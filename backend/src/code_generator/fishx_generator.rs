use crate::code_generator::code_generator::CodeGenerator;
use crate::code_generator::utils::value_to_js;
use crate::models::page::{Element, Page, PageContent};
use std::collections::HashSet;
use std::path::PathBuf;

const REPLACEMENT_CHARACTER: &str = "##replace##";

pub struct FishxGenerator;

impl CodeGenerator for FishxGenerator {
    async fn export_code(
        &self,
        code_dir: std::path::PathBuf,
        page_list: Vec<Page>,
    ) -> Result<(), String> {
        // 遍历页面列表并导出每个页面
        let mut index = 1;
        for page in &page_list {
            self.export_page(index, code_dir.clone(), page).await?;
            index += 1;
        }

        handle_routes(code_dir, page_list.len(), &page_list).await;

        Ok(())
    }

    async fn export_page(&self, index: usize, code_dir: PathBuf, page: &Page) -> Result<(), String> {
        // 存储生成的组件字符
    
        let mut components = Vec::new();
        // 存储组件类型
        let mut import_components_types = HashSet::new();
    
        // TODO 处理页面级别的事件和属性
    
        // TODO 处理事件
    
        // page.page_data string 转JSON
        let page_data: PageContent = match serde_json::from_str(&page.page_data) {
            Ok(data) => data,
            Err(e) => {
                // 记录错误信息并返回错误
                return Err(format!("解析 page_data 失败: {}", e));
            }
        };
    
        // 遍历页面元素
        for element in &page_data.elements {
            // import_components_types 传入到 generate_component_string 方法内部去调用
            let component_str =
                generate_component_string(&element, &page_data, &mut import_components_types);
            import_components_types.insert(element.type_name.clone());
            components.push(component_str);
        }
    
        // 生成 antd 导入语句
        let mut sorted_types: Vec<String> = import_components_types.into_iter().collect();
        sorted_types.sort();
        let antd_import = if !sorted_types.is_empty() {
            format!(
                "import {{ {} }} from '@/components';\n",
                sorted_types.join(", ")
            )
        } else {
            String::new()
        };
    
        // 生成最终的代码, 组件名称 Pageindex
        let comp_name = format!("Page{}", index);
    
        // 需要将 page 的 page_data 设置成 JSON对之后
    
        let page_json = serde_json::to_string_pretty(&page_data).unwrap();
    
        let output = format!(
            r#"import React, {{ useState, useEffect }} from 'react';
    import {{ PageWrapper }} from '@components/PageWrapper';
    {antd_import}
    import {{ usePageStore }} from '@/stores/pageStore';
    import {{ useShallow }} from 'zustand/react/shallow';
    import {{ Spin }} from 'antd';
    function {compName}() {{
    
      const [loading, setLoading] = useState(true)
      const {{ savePageInfo }} = usePageStore(
        useShallow((state) => {{
          return {{
            savePageInfo: state.savePageInfo,
        }};
        }}),
      );
    
      useEffect(() => {{
        savePageInfo({{
        "id": "{page_id}",
        "pageData": {page_str}
        }});
        setLoading(false);
      }}, []);
    
      if (loading) {{
        return <Spin />;
      }}
    
      return (
        <PageWrapper>
          {components}
        </PageWrapper>
      );
    }}
    
    export default {compName};"#,
            compName = &comp_name,
            components = components.join("\n      "),
            antd_import = antd_import,
            page_str = page_json,
            page_id = page.id,
        );
    
        println!("{}", output);
    
        // 将生成的代码写入 GeneratedPage.tsx 文件
        // TODO: 约定一下输出文件的路径
        let fe_page_dir = code_dir
            .join("app")
            .join("src")
            .join("pages")
            .join(&comp_name);
        if !fe_page_dir.exists() {
            tokio::fs::create_dir_all(&fe_page_dir).await;
        }
    
        tokio::fs::write(fe_page_dir.join("index.tsx"), output).await;
        Ok(())
    }
    
}

pub async fn handle_routes(code_dir: PathBuf, page_len: usize, page_list: &Vec<Page>) {
    let root_code_path = code_dir.join("app").join("src");
    // 读取路由文件
    let routes_path = root_code_path.join("config").join("routes.ts");

    let menus_path = root_code_path.join("config").join("menu.ts");

    // 确保config目录存在
    if !routes_path.parent().unwrap().exists() {
        tokio::fs::create_dir_all(routes_path.parent().unwrap()).await;
    }

    // 读取现有路由配置或创建新的
    let routes_content = if routes_path.exists() {
        tokio::fs::read_to_string(&routes_path).await.unwrap_or_else(|_| String::new())
    } else {
        String::new()
    };

    let menus_content = if menus_path.exists() {
        tokio::fs::read_to_string(&menus_path).await.unwrap_or_else(|_| String::new())
    } else {
        String::new()
    };

    let mut new_routes = String::new();

    let mut new_menus = String::new();

    for index in 0..page_len {
        let comp_name = format!("Page{}", index + 1);
        let page = page_list.get(index);
        // TODO 如果页面也有 path 则 path 设置成页面的配置
        // 如果 page 对象 path 字段有值，则取path 没有则取comp_namel
        let page_path = page
            .and_then(|page| page.path.clone()) // 解包 page 和 page.path
            .filter(|path| !path.is_empty()) // 过滤掉空字符串
            // unwrap_or / + comp_name.clone()
            .unwrap_or(format!("/{}", comp_name));

        // 处理 Fishx 模版路由信息
        let new_route = format!(
            "\n      {{ path: '{route_path}', component: './{route_name}' }},",
            route_path = page_path,
            route_name = &comp_name
        );

        new_routes.push_str(&new_route);

        // 处理 Fishx 模版菜单信息
        let new_menu = format!(
            "\n  {{\n    path: '{route_path}',\n    name: '{route_name}',\n  }},",
            route_path = page_path,
            route_name = &comp_name
        );

        new_menus.push_str(&new_menu);
    }

    // 在数组结束前插入新路由
    let updated_routes = if routes_content.contains(REPLACEMENT_CHARACTER) {
        routes_content.replace(REPLACEMENT_CHARACTER, &format!("{}\n", new_routes))
    } else {
        routes_content
    };

    // 在数组结束前插入新菜单
    let updated_menus = if menus_content.contains(REPLACEMENT_CHARACTER) {
        menus_content.replace(REPLACEMENT_CHARACTER, &format!("{}\n", new_menus))
    } else {
        menus_content
    };

    // 写入更新后的路由配置
    tokio::fs::write(&routes_path, updated_routes).await;

    // 写入更新后的菜单配置
    tokio::fs::write(&menus_path, updated_menus).await;

}

fn generate_component_string(
    element: &Element,
    page_data: &PageContent,
    import_components_types: &mut HashSet<String>,
) -> String {
    let component_type = &element.type_name;
    import_components_types.insert(component_type.clone());
    // let text = String::from("Typography.Text");
    // if component_type == "Text" {
    //     component_type = &text;
    // }

    let element_id = &element.id;
    let child_elements: &Vec<Element> = &element.elements;

    // 获取组件的配置
    let js_config = page_data
        .elements_map
        .get(element_id)
        .map(|config| value_to_js(&config.config))
        .unwrap_or_else(|| "null".to_string());

    // 递归处理子组件
    let child_components: Vec<String> = child_elements
        .iter()
        .map(|child| generate_component_string(child, page_data, import_components_types))
        .collect();

    // 如果 component_type 是 Button 也需要特殊处理

    if child_components.is_empty() {
        format!("<{component_type} config={{{js_config}}} />")
    } else {
        let children_str = child_components.join("\n");
        format!("<{component_type} config={{{js_config}}}>\n{children_str}\n</{component_type}>")
    }
}
