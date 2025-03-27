use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct FontMeta {
    pub postscript_name: String,
    pub family: String,
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Img,
    Font,
    Js,
    Attachment,
    Other,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Img => "img",
            ResourceType::Font => "font",
            ResourceType::Js => "js",
            ResourceType::Attachment => "attachment",
            ResourceType::Other => "other",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceQueryParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub keyword: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperResourceGroupParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub new_group_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub file_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub last_modified_time: String,
    pub file_size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceGroupInfo {
    pub name: String,
    pub path: String,
    pub last_modified_time: String,
    pub resources: Vec<ResourceInfo>,
    // 默认分组
    pub default_group: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameResource {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub resource_name: String,
    pub new_resource_name: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DeleteResource {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub resource_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResourceParams {
    // 原始文件本地路径
    pub file_path: String,
    pub old_file_path: Option<String>,
}
