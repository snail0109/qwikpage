use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiCollection {
    pub apis: HashMap<Uuid, Api>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    pub method: HttpMethod,
    pub url: String,
    pub source_type: SourceType,
    pub params: Vec<Param>,
    pub content_type: String,
    pub replace_data: ReplaceData,
    #[serde(default)]
    pub is_cors: bool,
    pub result: ApiResult,
    pub tips: ApiTips,
    pub id: Uuid,
    pub name: String,
    pub stg_api: String,
    pub source_field: SourceField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    // 可根据需要扩展其他方法
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Json,
    Xml,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReplaceData {
    Merge,
    Replace,
    // 其他可能的替换策略
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResult {
    pub code: String,
    pub code_value: Option<serde_json::Value>,
    pub data: String,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTips {
    pub success: String,
    pub fail: String,
    pub is_success: bool,
    #[serde(default)]
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceField {
    #[serde(rename = "type")]
    pub field_type: SourceFieldType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceFieldType {
    Static,
    Dynamic,
    // 其他可能的类型
}