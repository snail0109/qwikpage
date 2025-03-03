use crate::code_generator::code_generator::CodeGenerator;
use crate::code_generator::utils::value_to_js;
use crate::models::page::{Element, Page, PageContent};
use crate::utils::get_app_root_resource_dir;
use futures::future::BoxFuture;
use log::info;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

type Result<T> = std::result::Result<T, String>;

const REPLACEMENT_CHARACTER: &str = "##replace##";

const ROUTE_TEMPLATE: &str = r#"
  {
    path: '{path}',
    component: './{component}',
    exact: true,
  },"#;

const MENU_TEMPLATE: &str = r#"
  {
    path: '{route_path}',
    name: '{route_name}',
  },"#;

const REACT_COMPONENT_TEMPLATE: &str = r#"
import React, { useState, useEffect } from 'react';
import { PageWrapper } from '@components/PageWrapper';
{antd_import}
import { usePageStore } from '@/stores/pageStore';
import { useShallow } from 'zustand/react/shallow';
import { Spin } from 'antd';

function {compName}() {
  const [loading, setLoading] = useState(true);
  const { savePageInfo } = usePageStore(
    useShallow((state) => ({
      savePageInfo: state.savePageInfo,
    }))
  );

  useEffect(() => {{
    savePageInfo({
      id: "{page_id}",
      pageData: {page_str}
    });
    setLoading(false);
  }}, []);

  if (loading) return <Spin />;

  return (
    <PageWrapper>
      {components}
    </PageWrapper>
  );
}

export default {compName};
"#;

pub struct FishxGenerator;

impl CodeGenerator for FishxGenerator {
    async fn export_code(
        &self,
        project_id: &str,
        code_dir: &PathBuf,
        page_list: Vec<Page>,
    ) -> Result<()> {
        // 遍历页面列表并导出每个页面
        for (index, page) in page_list.iter().enumerate() {
            self.export_page(index + 1, code_dir.clone(), page, project_id)
                .await?;
        }

        handle_routes(&code_dir, &page_list).await?;

        Ok(())
    }

    async fn export_page(
        &self,
        index: usize,
        code_dir: PathBuf,
        page: &Page,
        project_id: &str,
    ) -> Result<()> {
        // 处理 resource
        let resource_path = get_app_root_resource_dir().join(project_id);
        let resource_path_str = resource_path.to_str().unwrap();
        // page_data_str 里面如果有  resource_path 则替换 为 /
        let page_data_str = &page.page_data.replace(resource_path_str, "/");

        // TODO 处理事件

        // page.page_data string 转JSON
        let page_data: PageContent = serde_json::from_str(&page_data_str)
            .map_err(|e| format!("解析 page_data 失败: {}", e))?;

        let (components, imports) = generate_components(&page_data.elements, &page_data);

        let comp_name = format!("Page{}", index);

        let output = generate_page_code(&comp_name, &page.id, &page_data, &components, &imports);

        // 将生成的代码写入 GeneratedPage.tsx 文件
        let fe_page_dir = code_dir
            .join("app")
            .join("src")
            .join("pages")
            .join(&comp_name);

        create_dir_if_not_exists(&fe_page_dir).await?;

        write_file(fe_page_dir.join("index.tsx"), &output).await?;
        Ok(())
    }

    async fn export_resources(&self, project_id: &str, code_dir: &PathBuf) -> Result<()> {
        // 获取项目资源目录
        let prj_res_dir = get_app_root_resource_dir().join(project_id);

        if !prj_res_dir.exists() {
            info!("Resource directory does not exist: {:?}", prj_res_dir);
            return Ok(());
        }

        // 目标目录：code_dir 的 public 子目录
        let public_dir = code_dir.join("app").join("public");

        // 确保目标目录存在
        async_fs::create_dir_all(&public_dir)
            .await
            .map_err(|e| format!("Failed to create target directory: {}", e))?;

        // 读取资源目录内容
        let mut entries = async_fs::read_dir(&prj_res_dir)
            .await
            .map_err(|e| format!("Failed to read resource directory: {}", e))?;

        // 复制目录内容
        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let source_path = entry.path();
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|_| format!("Invalid file name for path: {:?}", source_path))?;
            let target_path = public_dir.join(file_name);

            if source_path.is_dir() {
                // 如果是目录，递归复制
                copy_directory_recursive(&source_path, &target_path).await?;
            } else {
                // 如果是文件，直接复制
                async_fs::copy(&source_path, &target_path)
                    .await
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }

        Ok(())
    }
}

pub async fn handle_routes(code_dir: &PathBuf, pages: &Vec<Page>) -> Result<()> {
    let root_path = code_dir.join("app").join("src");
    let config_dir = root_path.join("config");

    let mut routes = String::new();

    let mut menus = String::new();

    for (index, page) in pages.iter().enumerate() {
        let comp_name = format!("Page{}", index + 1);
        // 如果 page 对象 path 字段有值，则取path 没有则取comp_namel
        let default_path = format!("/{}", comp_name);
        let path = page.path.as_deref().unwrap_or(&default_path);

        // 处理 Fishx 模版路由信息

        routes.push_str(
            &ROUTE_TEMPLATE
                .replace("{path}", &path)
                .replace("{component}", &comp_name),
        );

        // 处理 Fishx 模版菜单信息
        menus.push_str(
            &MENU_TEMPLATE
                .replace("{route_path}", &path)
                .replace("{route_name}", &comp_name),
        );
    }

    update_config_file(config_dir.join("routes.ts"), REPLACEMENT_CHARACTER, &routes).await?;
    update_config_file(config_dir.join("menu.ts"), REPLACEMENT_CHARACTER, &menus).await?;

    Ok(())
}

fn generate_import_statement(components: &[String]) -> String {
    if components.is_empty() {
        return String::new();
    }

    format!(
        "import {{ {} }} from '@/components';\n",
        components.join(", ")
    )
}

fn generate_components(
    elements: &[Element],
    page_data: &PageContent,
) -> (Vec<String>, Vec<String>) {
    let mut components = Vec::new();
    let mut imports = HashSet::new();

    for element in elements {
        let (component_str, types) = process_element(element, page_data);
        components.push(component_str);
        imports.extend(types);
    }

    let mut sorted_imports: Vec<String> = imports.into_iter().collect();
    sorted_imports.sort();
    (components, sorted_imports)
}

fn process_element(element: &Element, page_data: &PageContent) -> (String, HashSet<String>) {
    let mut imports = HashSet::new();
    let component_str = generate_component(element, page_data, &mut imports);
    (component_str, imports)
}

fn generate_component(
    element: &Element,
    page_data: &PageContent,
    imports: &mut HashSet<String>,
) -> String {
    imports.insert(element.type_name.clone());

    let config = page_data
        .elements_map
        .get(&element.id)
        .map(|c| value_to_js(&c.config))
        .unwrap_or_else(|| "null".into());

    let children: Vec<String> = element
        .elements
        .iter()
        .map(|e| generate_component(e, page_data, imports))
        .collect();

    if children.is_empty() {
        format!("<{} config={{{}}} />", element.type_name, config)
    } else {
        format!(
            "<{} config={{{}}}>\n{}\n</{}>",
            element.type_name,
            config,
            children.join("\n"),
            element.type_name
        )
    }
}

fn generate_page_code(
    comp_name: &str,
    page_id: &str,
    page_data: &PageContent,
    components: &[String],
    imports: &[String],
) -> String {
    let antd_import = generate_import_statement(&imports);

    let page_json = serde_json::to_string_pretty(page_data).unwrap_or_else(|_| "{}".into());

    let output = REACT_COMPONENT_TEMPLATE
        .replace("{antd_import}", &antd_import)
        .replace("{compName}", &comp_name)
        .replace("{page_id}", &page_id)
        .replace("{page_str}", &page_json)
        .replace("{components}", &components.join("\n      "));
    output
}

async fn create_dir_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        async_fs::create_dir_all(path)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }
    Ok(())
}

async fn write_file(path: PathBuf, content: &str) -> Result<()> {
    async_fs::write(&path, content)
        .await
        .map_err(|e| format!("写入文件失败: {}: {}", path.display(), e))
}

async fn read_file_if_exists(path: &Path) -> Result<String> {
    match async_fs::read_to_string(path).await {
        Ok(c) => Ok(c),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(format!("读取文件失败: {}: {}", path.display(), e)),
    }
}

async fn update_config_file(path: PathBuf, marker: &str, content: &str) -> Result<()> {
    let original = read_file_if_exists(&path).await?;
    let updated = original.replace(marker, &format!("{}\n", content));
    write_file(path, &updated).await
}

// 递归复制目录
fn copy_directory_recursive(
    source_dir: &PathBuf,
    target_dir: &PathBuf,
) -> BoxFuture<'static, Result<()>> {
    let source_dir = source_dir.clone();
    let target_dir = target_dir.clone();
    
    Box::pin(async move {
        // 确保目标目录存在
        async_fs::create_dir_all(&target_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;

        // 读取源目录内容
        let mut entries = async_fs::read_dir(source_dir)
            .await
            .map_err(|e| format!("读取目录失败: {}", e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let source_path = entry.path();
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|_| format!("无效的文件名: {:?}", source_path))?;
            let target_path = &target_dir.join(file_name);

            if source_path.is_dir() {
                // 递归复制子目录
                copy_directory_recursive(&source_path, &target_path).await?;
            } else {
                // 复制文件
                async_fs::copy(&source_path, &target_path)
                    .await
                    .map_err(|e| format!("复制文件失败: {}", e))?;
            }
        }

        Ok(())
    })
}