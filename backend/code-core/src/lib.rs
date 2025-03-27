pub mod ffi;

use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use thiserror::Error;
use anyhow::Error;

#[derive(Serialize, Deserialize)]
pub struct GeneratedArtifact {
    pub file_path: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteInfo {
    pub path: String,
    pub name: String,
    pub component_path: String,
}


#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Template error: {0}")]
    Template(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub trait CodeGenerator: Send + Sync {
    fn init_project(&self, options: &GeneratorOptions) -> Result<Vec<GeneratedArtifact>, Error>;
    fn generate_code(&self) -> Result<Vec<GeneratedArtifact>, Error>;
    fn generate_page(&self, config: &PageConfig, route_list: &mut Vec<RouteInfo>) -> Result<GeneratedArtifact, Error>;
}

// FFI兼容的类型转换
#[derive(Serialize, Deserialize)]
pub struct FfiResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> From<Result<T, GeneratorError>> for FfiResult<T> {
    fn from(result: Result<T, GeneratorError>) -> Self {
        match result {
            Ok(data) => FfiResult {
                success: true,
                data: Some(data),
                error: None,
            },
            Err(e) => FfiResult {
                success: false,
                data: None,
                error: Some(e.to_string()),
            },
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorOptions {
    pub project_name: String,
    pub output_dir: PathBuf,
    pub version: String,
    pub package_manager: String, // npm/yarn/pnpm
    pub page_list: Vec<PageConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageConfig {
    pub id: String,
    pub name: String,
    pub page_data: String,
    pub path: String,
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


pub fn hello() {
    println!("hello core")
}