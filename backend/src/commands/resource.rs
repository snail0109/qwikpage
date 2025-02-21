use tauri::command;

use crate::models::resource::{ResourceConfig, ResourceInfo, ResourceQueryParams};

use super::cmd_response::CmdResponse;

#[command]
pub async fn load_resource(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceInfo>> {
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}
