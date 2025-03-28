pub mod ffi;
pub mod types;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use types::{
    generator::{GeneratedArtifact, GeneratorError, GeneratorOptions},
    page::{Element, ElementObj, MergedElement, Page, PageContent},
    route::RouteInfo,
};

use anyhow::Error;

pub trait CodeGenerator: Send + Sync {
    fn init_project(&self, options: &GeneratorOptions) -> Result<Vec<GeneratedArtifact>, Error>;
    fn generate_code(&self) -> Result<Vec<GeneratedArtifact>, Error>;
    fn generate_page(
        &self,
        config: &Page,
        route_list: &mut Vec<RouteInfo>,
    ) -> Result<GeneratedArtifact, Error>;
}



// 模板数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
    pub components: Vec<MergedElement>,
}

impl TemplateData {
    pub fn from_json(json_str: &str) -> Result<Self, Error> {
        let data: PageContent = serde_json::from_str(json_str)?;
        let components = merge_element(&data.elements, &data.elements_map);
        println!("components {:?}", components);
        Ok(TemplateData { components })
    }
}

pub fn merge_element(
    elements: &Vec<Element>,
    elements_map: &HashMap<String, ElementObj>,
) -> Vec<MergedElement> {
    let mut merged_elements = Vec::new();

    // 遍历 elements，尝试从 elements_map 中找到对应的元素进行合并
    for element in elements {
        if let Some(element_obj) = elements_map.get(&element.id) {
            // 合并逻辑（这里只是简单的例子，具体合并规则可以根据需要修改）
            let child_elements = &element.elements;
            let mut merged_child_elements = Vec::new();
            // child_elements 不为空时，继续递归合并
            if !child_elements.is_empty() {
                merged_child_elements = merge_element(&child_elements, elements_map);
            }
            let merged = MergedElement {
                id: element.id.clone(),
                parent_id: element.parent_id.clone(),
                type_name: element.type_name.clone(),
                name: element.name.clone(),
                elements: merged_child_elements,
                config: element_obj.config.clone(),
                events: element_obj.events.clone(),
                methods: element_obj.methods.clone(),
            };
            merged_elements.push(merged);
        } else {
            // 处理没有找到匹配项的情况（如果需要）
            eprintln!("Warning: No matching element found for id: {}", element.id);
        }
    }
    println!("merged_elements: {:?}", merged_elements.len());
    merged_elements
}
