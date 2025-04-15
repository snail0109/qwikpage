use std::collections::HashMap;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub name: String,           // 页面名称
    pub path: Option<String>,   // 页面路由 TODO: 去掉Option
    pub remark: Option<String>, // 页面描述
    pub page_data: String,
    pub created_at: String,
    pub updated_at: String,
    pub project_id: String, // 保留冗余，方便查询
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Element {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub type_name: String, // 组件类型
    pub name: String,
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElementObj {
    pub config: Value,
    pub events: Vec<Event>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interceptor {
    pub headers: Vec<Header>,
    pub timeout: u32,
    #[serde(rename = "timeoutErrorMessage")]
    pub timeout_error_message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageContent {
    pub elements: Vec<Element>,
    #[serde(rename = "elementsMap")]
    pub elements_map: HashMap<String, ElementObj>,
    pub apis: HashMap<Uuid, Value>,
    pub interceptor: Option<Interceptor>,
}

// 事件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub value: String,
    pub name: String,
}

// methods
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Method {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MergedElement {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub type_name: String, // 组件类型
    pub name: String,
    pub elements: Vec<MergedElement>,
    pub config: Value,
    pub events: Vec<Event>,
    pub methods: Vec<Method>,
}

// ApiConfig 结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    #[serde(rename = "apiUrl")]
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageData {
    pub apis: HashMap<String, ApiConfig>,
}
