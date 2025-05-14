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
    // TODO: 生成 Template
    // 处理页面数据
    let page_data = &page.page_data;

    // 合并后的组件
    let template_data = match TemplateData::from_json(page_data) {
        Ok(data) => data,
        Err(e) => {
            return Err(Error::msg(format!("Failed to parse template data: {}", e)));
        }
    };
    let mut page_variables = String::new();
    let mut loop_variables = Vec::new();

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
    }

    // Vue页面一：处理页面数据
    let replacement = format!("const pageInfo = {};", page_json);

    // Vue页面二：处理页面级变量
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
            }
        }
    }

    let result = VIEW_TEMPLATE
        .replace("{{vue_template}}", &vue_template)
        .replace("{{pageVariables}}", &page_variables)
        .replace("{{ pageInfo }}", &replacement);
    write_file(file_path.clone(), &result)?;
    format_vue_file(file_path.clone());
    Ok(GeneratedArtifact {
        file_path: file_path.to_string_lossy().to_string(),
        content: result,
    })
}

// 类型映射
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
    parent_loop_vfor: Option<(String, String)>,
) -> (String, Vec<(String, String)>) {
    let indent_str = "  ";
    let tag = map_type(&element.type_name);

    // 处理 v-for 相关属性
    let mut v_for_attr = String::new();
    let mut key_attr = String::new();

    // 如果父级是 Loop，当前元素加 v-for
    if let Some((vfor_expr, key_field)) = parent_loop_vfor {
        // FIXME
        v_for_attr = format!(r#" v-for="item in {}""#, vfor_expr);
        key_attr = format!(r#" :key="item.{}""#, key_field);
    }

    let mut loop_variables = Vec::new();

    // 如果当前是 Loop 组件，准备传递给子元素的 v-for 信息
    let mut next_loop_vfor: Option<(String, String)> = None;

    if element.type_name == "Loop" {
        if let Some(api) = element.config.get("api") {
            if let Some(source_type) = api.get("sourceType").and_then(|v| v.as_str()) {
                match source_type {
                    "json" => {
                        // 变量名
                        let loop_var_name = format!("{}ApiSource", element.id);
                        // 变量值
                        let source_val = api
                            .get("source")
                            .map(|v| v.to_string())
                            .unwrap_or("[]".to_string());
                        loop_variables.push((loop_var_name.clone(), source_val.clone()));
                        next_loop_vfor = Some((loop_var_name.clone(), "id".to_string()));
                    }
                    "variable" => {
                        if let Some(name) = api.get("name") {
                            if let Some(var_obj) = name.as_object() {
                                if let Some(Value::String(val)) = var_obj.get("value") {
                                    // 页面变量 context.variable.
                                    let last = val.split('.').last().unwrap_or("data").to_string();
                                    next_loop_vfor = Some((last, "id".to_string()));
                                    // 循环变量 context.forEachValue.Loop_top.item.cards || []
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        // 取 rowKey 字段
        if let Some(props) = element.config.get("props") {
            if let Some(row_key) = props.get("rowKey").and_then(|v| v.as_str()) {
                if let Some((vfor_expr, _)) = next_loop_vfor {
                    next_loop_vfor = Some((vfor_expr, row_key.to_string()));
                }
            }
        }
    }

    let mut children_parts = Vec::new();
    let mut children_vars = Vec::new();
    for child in &element.elements {
        let (child_str, child_vars) = if element.type_name == "Loop" {
            generate_template(child, next_loop_vfor.clone())
        } else {
            generate_template(child, None)
        };
        children_parts.push(child_str);
        children_vars.extend(child_vars);
    }
    let children = children_parts.join("\n");

    let processed_config = bind_variable_in_config(&element.config);
    let config_js = value_to_js_object_literal(&processed_config);
    let attrs = format!(r#" :config="{}""#, config_js);
    // let evts = events_to_attrs(&element.events);
    let evts = "";
    // 返回 (模板字符串, 变量收集)
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

// 事件转Vue绑定
// fn events_to_attrs(events: &[Event]) -> String {
//     events.iter()
//         .map(|e| format!(r#" @{}="{}""#, e.name, e.handler))
//         .collect::<Vec<_>>()
//         .join("")
// }

fn extract_prop_value(value: &serde_json::Value) -> serde_json::Value {
    // 如果是对象且有 type 字段
    if let serde_json::Value::Object(obj) = value {
        if let Some(serde_json::Value::String(type_str)) = obj.get("type") {
            match type_str.as_str() {
                "static" => obj.get("value").cloned().unwrap_or(serde_json::Value::Null),
                "variable" | "globalVariable" => {
                    // 用字符串代表 renderFormula(variableObj.value)
                    if let Some(var_val) = obj.get("value") {
                        // FIXME
                        serde_json::Value::Null
                    } else {
                        serde_json::Value::Null
                    }
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
pub fn bind_variable_in_config(config: &Value) -> Value {
    match config {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map {
                if k == "props" {
                    // props 是个对象，对每个 value 做 extract_prop_value
                    if let Value::Object(props_map) = v {
                        let mut new_props = Map::new();
                        for (pk, pv) in props_map {
                            new_props.insert(pk.clone(), extract_prop_value(pv));
                        }
                        new_map.insert(k.clone(), Value::Object(new_props));
                    } else {
                        // props 不是对象，原样返回
                        new_map.insert(k.clone(), v.clone());
                    }
                } else {
                    // 递归处理其他字段
                    new_map.insert(k.clone(), bind_variable_in_config(v));
                }
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(bind_variable_in_config).collect()),
        _ => config.clone(),
    }
}
