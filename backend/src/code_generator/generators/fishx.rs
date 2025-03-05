use std::collections::HashSet;
use crate::code_generator::error::Result;
use crate::code_generator::utils::value_to_js;
use crate::models::page::{Element, PageContent};

use crate::code_generator::config::GeneratorConfig;
use crate::code_generator::generators::CodeGenerator;
use crate::code_generator::templates::{react, REPLACEMENT_CHARACTER};
use crate::models::page::Page;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::code_generator::utils::{
    create_dir_if_not_exists, download_template, export_resources, process_page_data,
    replace_template, update_config_file, write_file,
};

const BASE_DIR_NAME: &str = "fishx";  

const PUBLIC_RESPIRCE_PATH: &str = "fishx/public";  

/// Fishx框架代码生成器
#[derive(Debug)]

pub struct FishxGenerator;

impl CodeGenerator for FishxGenerator {
    async fn export_code(&self, config: &GeneratorConfig, page_list: Vec<Page>) -> Result<()> {
        // 遍历页面列表并导出每个页面
        for (index, page) in page_list.iter().enumerate() {
            self.export_page(index + 1, config, page).await?;
        }

        // 处理路由和菜单
        self.handle_routes(&config.output_dir, &page_list).await?;

        Ok(())
    }

    async fn export_page(&self, index: usize, config: &GeneratorConfig, page: &Page) -> Result<()> {
        // 处理页面数据
        let page_data = process_page_data(&page.page_data, &config.project_id)?;

        // 生成组件
        let (components, imports) =
            self.generate_components(&page_data.elements, &page_data)?;

        // 组件代码
        let components_str = components.join("\n");

        // 导入语句
        let antd_import = self.generate_import_statement(&imports);

        // 组件名称
        let comp_name = format!("Page{}", index);

        // 替换模板变量
        let mut replacements = HashMap::new();
        replacements.insert("compName", &comp_name);
        replacements.insert("page_id", &page.id);
        let page_str = serde_json::to_string(&page_data).unwrap_or_default();
        replacements.insert("page_str", &page_str);
        replacements.insert("components", &components_str);
        replacements.insert("antd_import", &antd_import);

        let output = replace_template(react::COMPONENT, &replacements);

        // 创建页面目录
        let pages_dir = config
            .output_dir
            .join(BASE_DIR_NAME)
            .join("src")
            .join("pages")
            .join(&comp_name);
        create_dir_if_not_exists(&pages_dir).await?;

        // 写入文件
        write_file(pages_dir.join("index.tsx"), &output).await?;

        Ok(())
    }

    async fn export_resources(&self, config: &GeneratorConfig) -> Result<()> {
        // 使用基础生成器的资源导出功能
        export_resources(&config.project_id, &config.output_dir, PUBLIC_RESPIRCE_PATH).await
    }

    async fn download_template(&self, config: &GeneratorConfig) -> Result<()> {
        // 使用基础生成器的下载模板功能
        download_template(&config.template_url, &config.output_dir).await
    }

    fn name(&self) -> &'static str {
        "Fishx"
    }
    
    fn generate_components(&self, elements: &[crate::models::page::Element], page_data: &crate::models::page::PageContent) -> Result<(Vec<String>, Vec<String>)> {
        let mut components = Vec::new();
        let mut imports = HashSet::new();

        for element in elements {
            let (component_str, types) = self.process_element(element, page_data)?;
            components.push(component_str);
            imports.extend(types);
        }

        let mut sorted_imports: Vec<String> = imports.into_iter().collect();
        sorted_imports.sort();
        Ok((components, sorted_imports))
    }
    
    fn generate_import_statement(&self, components: &[String]) -> String {
        if components.is_empty() {
            return String::new();
        }

        format!(
            "import {{ {} }} from '@/components';\n",
            components.join(", ")
        )
    }
}

impl FishxGenerator {
    /// 处理路由和菜单
    async fn handle_routes(&self, output_dir: &PathBuf, pages: &[Page]) -> Result<()> {
        let config_dir = output_dir.join(BASE_DIR_NAME).join("src").join("config");

        let mut routes = String::new();
        let mut menus = String::new();

        for (index, page) in pages.iter().enumerate() {
            let comp_name = format!("Page{}", index + 1);
            let default_path = format!("/{}", comp_name.to_lowercase());
            let path = page.path.as_deref().unwrap_or(&default_path);

            // 路由
            let mut route_replacements = HashMap::new();
            // path 如果是* 处理 为 /
            if path == "*" {
                route_replacements.insert("path", "/");
            } else {
                route_replacements.insert("path", path);
            }
            route_replacements.insert("component", &comp_name);
            routes.push_str(&replace_template(react::ROUTE, &route_replacements));

            // 菜单
            let mut menu_replacements = HashMap::new();
            menu_replacements.insert("route_path", path);
            menu_replacements.insert("route_name", &comp_name);
            menus.push_str(&replace_template(react::MENU, &menu_replacements));
        }

        // 更新路由文件
        update_config_file(config_dir.join("routes.ts"), REPLACEMENT_CHARACTER, &routes).await?;

        // 更新菜单文件
        update_config_file(config_dir.join("menu.ts"), REPLACEMENT_CHARACTER, &menus).await?;

        Ok(())
    }

    fn process_element(&self, element: &Element, page_data: &PageContent) -> Result<(String, HashSet<String>)> {
        let mut imports = HashSet::new();
        let component_str = self.generate_component(element, page_data, &mut imports)?;
        Ok((component_str, imports))
    }

    fn generate_component(&self, element: &Element, page_data: &PageContent, imports: &mut HashSet<String>) -> Result<String> {
        imports.insert(element.type_name.clone());

        let config = page_data
            .elements_map
            .get(&element.id)
            .map(|c| value_to_js(&c.config))
            .unwrap_or_else(|| "null".into());

        let children: Vec<String> = element
            .elements
            .iter()
            .map(|e| self.generate_component(e, page_data, imports))
            .collect::<Result<Vec<String>>>()?;

        let component_str = if children.is_empty() {
            format!("<{} config={{{}}} />", element.type_name, config)
        } else {
            format!(
                "<{} config={{{}}}>\n{}\n</{}>",
                element.type_name,
                config,
                children.join("\n"),
                element.type_name
            )
        };
        
        Ok(component_str)
    }
}

