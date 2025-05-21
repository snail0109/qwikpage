use std::path::PathBuf;

use anyhow::Error;
use code_core::types::generator::GeneratedArtifact;
use code_core::types::page::{MergedElement, Page};
use code_core::TemplateData;
use serde_json::{self, json, Map, Value};

use crate::templates::VIEW_TEMPLATE;
use crate::utils::{format_vue_file, write_file};

/**
 * 生成 Vue 页面文件
 * @param output_dir 输出目录
 * @param file_name 文件名
 * @param page 页面信息
 * @return 生成的文件信息
 */
pub fn gen_view_file(
    output_dir: PathBuf,
    file_name: &str,
    page: &Page,
) -> Result<GeneratedArtifact, Error> {
    let file_path = output_dir.join("src/views").join(file_name);
    // 创建一个自定义的 JSON 对象
    let frontend_data = json!({
        "id": page.id,
        "name": page.name,
        "path": page.path,
        "remark": page.remark,
        "pageData": page.page_data,
    });
    // 将 JSON 转换为字符串
    let page_json = serde_json::to_string_pretty(&frontend_data)
        .map_err(|e| anyhow::anyhow!("序列化失败: {}", e))?;

    // 数据格式变动频繁, 先不限制具体类型, 直接使用 serde_json::Value
    let page_data_json: serde_json::Value =
        serde_json::from_str(&page.page_data).expect("Failed to parse pageData");

    // 合并后的组件
    let template_data = match TemplateData::from_json(&page.page_data) {
        Ok(data) => data,
        Err(e) => {
            return Err(Error::msg(format!("Failed to parse template data: {}", e)));
        }
    };

    // 页面 ref 变量
    let mut page_variables = String::new();
    // watch pinia store的变量，修改代码里面 ref 定义的变量
    let mut watch_variables = String::new();
    // 循环变量，循环作用在子组件上，需要从父组件传递循环变量
    let mut loop_variables = Vec::new();

    //  Vue 模板 和 变量
    let vue_template_and_vars = template_data
        .components
        .iter()
        .map(|element| generate_template(element, None))
        .collect::<Vec<_>>();

    let vue_template = vue_template_and_vars
        .iter()
        .map(|(tpl, _)| tpl.clone())
        .collect::<Vec<_>>()
        .join("\n");

    for (_, vars) in vue_template_and_vars {
        loop_variables.extend(vars);
    }

    // 注入 Loop 相关变量
    for (var_name, value) in loop_variables {
        // value 可能是 JSON字符串，去掉多余引号
        let value_str = if value.starts_with('"') && value.ends_with('"') {
            &value[1..value.len() - 1]
        } else {
            &value
        };
        page_variables.push_str(&format!("const {} = ref({});\n", var_name, value_str));
        // 生成 watch 代码
        watch_variables.push_str(&format!(
            r#"watch(
  () => pageState.value.page.pageData,
  (newVal) => {{
    if (newVal.variableData["{var_name}"]) {{
      {var_name}.value = newVal.variableData["{var_name}"];
    }}
  }},
  {{ deep: true }}
);
"#,
            var_name = var_name
        ));
    }

    // 处理页面数据
    let replacement = format!("const pageInfo = {};", page_json);

    // 处理页面级变量
    if let Some(variables) = page_data_json.get("variables").and_then(|v| v.as_array()) {
        for variable in variables {
            // 这里需要进一步检查 variable 是否是对象类型
            if let Some(var_obj) = variable.as_object() {
                let name = var_obj
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or_default();

                let default_value = var_obj
                    .get("defaultValue")
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "null".to_string());

                page_variables.push_str(&format!("const {} = ref({});\n", name, default_value));
                // 生成 watch 代码
                watch_variables.push_str(&format!(
                    r#"watch(
  () => pageState.value.page.pageData,
  (newVal) => {{
    if (newVal.variableData["{name}"]) {{
      {name}.value = newVal.variableData["{name}"];
    }}
  }},
  {{ deep: true }}
);
"#,
                    name = name
                ));
            }
        }
    }

    let result = VIEW_TEMPLATE
        .replace("{{vue_template}}", &vue_template)
        .replace("{{pageVariables}}", &page_variables)
        .replace("{{watch_variables}}", &watch_variables)
        .replace("{{ pageInfo }}", &replacement);

    write_file(file_path.clone(), &result)?;

    format_vue_file(file_path.clone());
    Ok(GeneratedArtifact {
        file_path: file_path.to_string_lossy().to_string(),
        content: result,
    })
}

// 组件标签映射
fn map_type(type_name: &str) -> &str {
    match type_name {
        "Input" => "q-input",
        "Button" => "q-button",
        "Card" => "q-card",
        "Checkbox" => "q-checkbox",
        "Col" => "q-col",
        "Flex" => "q-flex",
        "Form" => "q-form",
        "Icon" => "q-icon",
        "Image" => "q-image",
        "Link" => "q-link",
        "Select" => "q-select",
        "Switch" => "q-switch",
        "Text" => "q-text",
        "Grid" => "q-grid",
        "Loop" => "q-flex", // 特殊处理
        other => other,     // fallback
    }
}

// 递归生成Vue模板
fn generate_template(
    element: &MergedElement,
    parent_loop_vfor: Option<(String, String, String)>, // (vfor_expr, key_field, item_var)
) -> (String, Vec<(String, String)>) {
    let indent_str = "  ";
    let tag = map_type(&element.type_name);

    // 处理 v-for 相关属性
    let mut v_for_attr = String::new();
    let mut key_attr = String::new();
    let mut item_var = "item".to_string();

    // 如果父级是 Loop 循环组件，当前元素加 v-for
    if let Some((vfor_expr, key_field, parent_item_var)) = parent_loop_vfor {
        v_for_attr = format!(r#" v-for="{} in {}""#, parent_item_var, vfor_expr);
        key_attr = format!(r#" :key="{}.{}""#, parent_item_var, key_field);
        item_var = parent_item_var;
    }

    let mut loop_variables = Vec::new();

    // 如果当前是 Loop 循环组件，准备传递给子元素的 v-for 信息
    let mut child_element_vfor: Option<(String, String, String)> = None;

    if element.type_name == "Loop" {
        println!("处理循环组件");
        if let Some(api) = element.config.get("api") {
            if let Some(source_type) = api.get("sourceType").and_then(|v| v.as_str()) {
                match source_type {
                    "json" => {
                        let loop_var_name = format!("{}ApiSource", element.id);
                        let source_val = api
                            .get("source")
                            .map(|v| v.to_string())
                            .unwrap_or("[]".to_string());
                        loop_variables.push((loop_var_name.clone(), source_val.clone()));
                        let item_var_name = format!("{}_item", element.id);
                        child_element_vfor =
                            Some((loop_var_name.clone(), "id".to_string(), item_var_name));
                    }
                    "variable" => {
                        if let Some(name) = api.get("name") {
                            if let Some(var_obj) = name.as_object() {
                                if let Some(Value::String(val)) = var_obj.get("value") {
                                    let replaced = if val.starts_with("context.variable.") {
                                        // context.variable.xxx -> xxx
                                        val.trim_start_matches("context.variable.").to_string()
                                    } else if val.starts_with("context.forEachValue.") {
                                        // context.forEachValue.Loop_top.item.cards -> Loop_top_item.cards
                                        let s = val.trim_start_matches("context.forEachValue.");
                                        let parts: Vec<&str> = s.split('.').collect();
                                        if parts.len() >= 3 {
                                            format!("{}_item.{}", parts[0], parts[2..].join("."))
                                        } else {
                                            s.to_string()
                                        }
                                    } else {
                                        val.clone()
                                    };
                                    let item_var_name = format!("{}_item", element.id);
                                    child_element_vfor =
                                        Some((replaced, "id".to_string(), item_var_name));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(props) = element.config.get("props") {
            if let Some(row_key) = props.get("rowKey").and_then(|v| v.as_str()) {
                if let Some((vfor_expr, _, item_var_name)) = &child_element_vfor {
                    child_element_vfor = Some((
                        vfor_expr.clone(),
                        row_key.to_string(),
                        item_var_name.clone(),
                    ));
                }
            }
        }
    }

    let mut children_parts = Vec::new();
    let mut children_vars = Vec::new();
    for child in &element.elements {
        let (child_str, child_vars) = if element.type_name == "Loop" {
            generate_template(child, child_element_vfor.clone())
        } else {
            generate_template(child, None)
        };
        children_parts.push(child_str);
        children_vars.extend(child_vars);
    }
    let children = children_parts.join("\n");

    // 处理 config，提取 text 字段
    let mut processed_config = bind_variable_in_config_with_item(&element.config, &item_var);

    // 提取 text 字段（如果有）
    let mut text_attr = String::new();
    if let Some(props) = processed_config.get_mut("props") {
        if let Some(props_map) = props.as_object_mut() {
            if let Some(text_value) = props_map.remove("text") {
                match &text_value {
                    serde_json::Value::String(s) => {
                        // 普通字符串，静态属性
                        text_attr = format!(r#" text="{}""#, s.replace('"', "&quot;"));
                    }
                    serde_json::Value::Object(obj) => {
                        // 对象，变量绑定
                        if let Some(serde_json::Value::String(var_name)) = obj.get("value") {
                            text_attr = format!(r#" :text="{}""#, var_name);
                        }
                    }
                    _ => {
                        // 其他类型，按变量绑定处理
                        let text_str = value_to_js_object_literal(&text_value);
                        text_attr = format!(r#" :text="{}""#, text_str);
                    }
                }
            }
        }
    }

    let config_js = value_to_js_object_literal(&processed_config);
    let attrs = format!(r#"{} :config="{}""#, text_attr, config_js);
    let evts = "";

    (
        if children.is_empty() {
            format!(
                "{indent_str}<{tag}{attrs}{evts} />",
                indent_str = indent_str,
                tag = tag,
                attrs = attrs,
                evts = evts
            )
        } else {
            format!(
                "{indent_str}<{tag}{attrs}{evts}{v_for_attr}{key_attr}>\n{children}\n{indent_str}</{tag}>",
                indent_str = indent_str,
                tag = tag,
                attrs = attrs,
                evts = evts,
                v_for_attr = v_for_attr,
                key_attr = key_attr,
                children = children
            )
        },
        {
            let mut all_vars = loop_variables;
            all_vars.extend(children_vars);
            all_vars
        },
    )
}

fn value_to_js_object_literal(value: &serde_json::Value) -> String {
    println!("value_to_js_object_literal: {:?}", value);
    match value {
        serde_json::Value::Object(map) => {
            let mut parts = Vec::new();
            for (k, v) in map {
                // key 直接输出（假设都是合法 JS 标识符）
                if k == "scopeCss" || k == "scopeStyle" || k == "api" || k == "events" {
                    continue; // 跳过 scopeCss 和 scopeStyle
                }
                let v_str = value_to_js_object_literal(v);
                parts.push(format!("{}:{}", k, v_str));
            }
            format!("{{{}}}", parts.join(","))
        }
        serde_json::Value::Array(arr) => {
            let items = arr
                .iter()
                .map(value_to_js_object_literal)
                .collect::<Vec<_>>();
            format!("[{}]", items.join(","))
        }
        serde_json::Value::String(s) => {
            // 用单引号包裹，并转义单引号
            let s = s.replace('\'', "\\'");
            format!("'{}'", s)
        }
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Null => "null".to_string(),
    }
}

// 变量名替换
fn extract_prop_value_with_item(value: &serde_json::Value, item_var: &str) -> serde_json::Value {
    println!("extract_prop_value_with_item: {:?}", value);
    // 如果是对象且有 type 字段
    if let serde_json::Value::Object(obj) = value {
        if let Some(serde_json::Value::String(type_str)) = obj.get("type") {
            match type_str.as_str() {
                "static" => obj.get("value").cloned().unwrap_or(serde_json::Value::Null),
                "variable" | "globalVariable" => {
                    // 保留原始对象，但 value 字段做变量名替换
                    let mut new_obj = obj.clone();
                    if let Some(serde_json::Value::String(var_val)) = obj.get("value") {
                        let replaced = if var_val.starts_with("context.variable.") {
                            // context.variable.xxx -> xxx
                            var_val.trim_start_matches("context.variable.").to_string()
                        } else if var_val.starts_with("context.forEachValue.") {
                            // context.forEachValue.Loop_top.item.cards -> Loop_top_item.cards
                            let s = var_val.trim_start_matches("context.forEachValue.");
                            let parts: Vec<&str> = s.split('.').collect();
                            if parts.len() >= 3 {
                                format!("{}_item.{}", parts[0], parts[2..].join("."))
                            } else {
                                s.to_string()
                            }
                        } else {
                            var_val.clone()
                        };
                        new_obj.insert("value".to_string(), serde_json::Value::String(replaced));
                    }
                    serde_json::Value::Object(new_obj)
                }
                _ => value.clone(),
            }
        } else {
            value.clone()
        }
    } else {
        value.clone()
    }
}

// 递归处理 config
pub fn bind_variable_in_config_with_item(config: &Value, item_var: &str) -> Value {
    println!("bind_variable_in_config_with_item: {:?}", config);
    match config {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map {
                if k == "props" {
                    if let Value::Object(props_map) = v {
                        let mut new_props = Map::new();
                        for (pk, pv) in props_map {
                            new_props
                                .insert(pk.clone(), extract_prop_value_with_item(pv, item_var));
                        }
                        new_map.insert(k.clone(), Value::Object(new_props));
                    } else {
                        new_map.insert(k.clone(), v.clone());
                    }
                } else {
                    new_map.insert(k.clone(), bind_variable_in_config_with_item(v, item_var));
                }
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(
            arr.iter()
                .map(|v| bind_variable_in_config_with_item(v, item_var))
                .collect(),
        ),
        _ => config.clone(),
    }
}
