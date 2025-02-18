use crate::models::page::{Element, Page, PageContent};
use log::info;
use reqwest;
use serde_json::Value;
use std::{collections::HashSet, fs, path::PathBuf};
use zip;

const REPLACEMENT_CHARACTER: &str = "##replace##";

// 将 JSON 值转换为 JavaScript 表示的字符串
fn value_to_js(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", escape_string(s)),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_js).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, value_to_js(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

// 转义字符串中的特殊字符
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn export_page(index: usize, code_dir: PathBuf, page: &Page) {
    // 存储生成的组件字符

    let mut components = Vec::new();
    // 存储组件类型
    let mut import_components_types = HashSet::new();

    // TODO 处理页面级别的事件和属性

    // TODO 处理事件

    // page.page_data string 转JSON
    let page_data: PageContent = serde_json::from_str(&page.page_data).unwrap();


    // 遍历页面元素
    for element in &page_data.elements {
        // import_components_types 传入到 generate_component_string 方法内部去调用
        let component_str =
            generate_component_string(&element, &page_data, &mut import_components_types);
        // if element.type_name == "Text" {
        //     import_components_types.insert(String::from("Typography"));
        // } else {
        //     import_components_types.insert(element.type_name.clone());
        // }
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
        fs::create_dir_all(&fe_page_dir).expect("failed to find code dir");
    }

    fs::write(fe_page_dir.join("index.tsx"), output).unwrap();
}

pub fn handle_routes(code_dir: PathBuf, page_len: usize, page_list: &Vec<Page>) {
    let root_code_path = code_dir.join("app").join("src");
    // 读取路由文件
    let routes_path = root_code_path.join("config").join("routes.ts");

    let menus_path = root_code_path.join("config").join("menu.ts");

    // 确保config目录存在
    if !routes_path.parent().unwrap().exists() {
        fs::create_dir_all(routes_path.parent().unwrap())
            .expect("failed to create config directory");
    }

    // 读取现有路由配置或创建新的
    let routes_content = if routes_path.exists() {
        fs::read_to_string(&routes_path).unwrap_or_default()
    } else {
        String::new()
    };

    let menus_content = if menus_path.exists() {
        fs::read_to_string(&menus_path).unwrap_or_default()
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
    fs::write(&routes_path, updated_routes).expect("failed to write routes file");

    // 写入更新后的菜单配置
    fs::write(&menus_path, updated_menus).expect("failed to write menus file");
}

pub fn download_temp(code_dir: &PathBuf) -> Result<(), String> {
    // 下载代码模板
    let template_url = String::from("https://fish.iwhalecloud.com/qwikpage-fishx/app.zip");
    let template_path = code_dir.join("fishx-template.zip");

    info!("下载代码模板......");
    let response =
        reqwest::blocking::get(template_url).map_err(|e| format!("下载模板失败: {}", e))?;
    let content = response
        .bytes()
        .map_err(|e| format!("读取响应内容失败: {}", e))?;

    info!("保存zip文件");
    fs::write(&template_path, content).map_err(|e| format!("保存模板文件失败: {}", e))?;

    info!("解压文件");
    let file = fs::File::open(&template_path).map_err(|e| format!("打开zip文件失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("读取zip文件失败: {}", e))?;

    info!("解压所有文件");
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("访问zip文件条目失败: {}", e))?;
        let outpath = code_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("创建目录失败: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| format!("创建父目录失败: {}", e))?;
                }
            }
            let mut outfile =
                fs::File::create(&outpath).map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("复制文件内容失败: {}", e))?;
        }
    }

    info!("删除zip文件");
    fs::remove_file(&template_path).map_err(|e| format!("删除zip文件失败: {}", e))?;

    // mac 下会生成 __MACOSX 文件
    #[cfg(target_os = "macos")]
    {
        let macosx_path = template_path.with_file_name("__MACOSX");
        if macosx_path.exists() {
            fs::remove_dir_all(macosx_path).map_err(|e| format!("删除macosx文件夹失败: {}", e))?;
        }
    }

    Ok(())
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
