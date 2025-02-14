use serde_json::{from_str, Value};
use std::{collections::HashSet, fs, path::PathBuf};

use crate::{
    models::page::{Page, PageContent},
    utils::get_app_root_dir,
};

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

pub fn export_page(index: usize , code_dir: PathBuf, page: Page) {
    // 存储生成的组件字符

    let mut components = Vec::new();
    // 存储组件类型
    let mut components_types = HashSet::new();

    // TODO 处理页面级别的事件和属性

    // TODO 处理事件

    // page.page_data string 转JSON
    let page_data: PageContent = serde_json::from_str(&page.page_data).unwrap();

    // 遍历页面元素
    for element in &page_data.elements {
        let element_id = &element.id;
        let component_type = &element.type_name;

        // 记录组件类型
        components_types.insert(component_type.clone());

        // 如果页面元素的配置存在，则生成组件字符串
        if let Some(config) = page_data.elements_map.get(element_id) {
            let js_config = value_to_js(&config.config);
            // TODO 透传 antd 组件的配置
            components.push(format!("<{component_type} config={{{js_config}}} />"));
        }
    }

    // 生成 antd 导入语句
    let mut sorted_types: Vec<String> = components_types.into_iter().collect();
    sorted_types.sort();
    let antd_import = if !sorted_types.is_empty() {
        format!("import {{ {} }} from 'antd';\n", sorted_types.join(", "))
    } else {
        String::new()
    };

    // 生成最终的代码, 组件名称 Pageindex
    let comp_name = format!("Page{}", index);

    let output = format!(
        r#"import React from 'react';
{antd_import}
function {compName}() {{
  return (
    <div>
      {components}
    </div>
  );
}}

export default {compName};"#,
        compName = comp_name,
        components = components.join("\n      "),
        antd_import = antd_import
    );

    println!("{}", output);

    // 将生成的代码写入 GeneratedPage.tsx 文件
    // TODO: 约定一下输出文件的路径
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code");
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir).expect("failed to create pages dir");
    }

    fs::write("./Page.tsx", output).unwrap();
}
