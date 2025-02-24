use tauri::command;

use crate::models::resource::{ResourceConfig, ResourceInfo, ResourceQueryParams};

use super::cmd_response::CmdResponse;

#[command]
pub async fn load_resource(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceInfo>> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}

// 创建资源分组(目录)
#[command]
pub async fn save_resource_group(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceInfo>> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}

// 删除资源分组(目录)
#[command]
pub async fn delete_resource_group(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceInfo>> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}

// 更新资源分组(目录)
#[command]
pub async fn update_resource_group(params: ResourceQueryParams) -> CmdResponse<(Vec<ResourceInfo>)> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}


// 导入资源
#[command]
pub async fn import_resource(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceInfo>> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}