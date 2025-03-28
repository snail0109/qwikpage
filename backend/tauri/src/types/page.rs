use std::collections::HashMap;

use code_core::Page;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::interceptor::Interceptor;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Element {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub type_name: String,
    pub name: String,
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElementConfig {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElementObj {
    pub config: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageContent {
    pub elements: Vec<Element>,
    #[serde(rename = "elementsMap")]
    pub elements_map: HashMap<String, ElementObj>,
    pub apis: HashMap<Uuid, Value>,
    pub interceptor: Option<Interceptor>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageList {
    pub list: Vec<Page>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageAddParams {
    pub name: String,
    pub path: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "pageData")]
    pub page_data: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageUpdateParams {
    pub id: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "pageData")]
    pub page_data: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageCopyParams {
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: String,
}