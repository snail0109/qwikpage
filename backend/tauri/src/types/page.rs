use code_core::Page;
use serde::{Deserialize, Serialize};

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