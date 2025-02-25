use std::path::PathBuf;

use log::info;
use tauri::command;

use crate::models::resource::{OperResourceGroupParams, ResourceConfig, ResourceGroupInfo, ResourceQueryParams, UploadParams};

use super::cmd_response::CmdResponse;

use font_kit;

#[command]
pub async fn load_resource(params: ResourceQueryParams) -> CmdResponse<Vec<ResourceGroupInfo>> {
    info!("load resource: {:?}", params);
    let config = ResourceConfig::load(params).await;
    CmdResponse::from(config)
}

// 创建资源分组(目录)
#[command]
pub async fn add_resource_group(params: OperResourceGroupParams) -> CmdResponse<bool> {
    info!("add resource group: {:?}", params);
    let config = ResourceConfig::add_resource_group(params).await;
    CmdResponse::from(config)
}

// 删除资源分组(目录)
#[command]
pub async fn delete_resource_group(params: OperResourceGroupParams) -> CmdResponse<bool> {
    info!("delete resource group: {:?}", params);
    let config = ResourceConfig::delete_resource_group(params).await;
    CmdResponse::from(config)
}

// 更新资源分组(目录)
#[command]
pub async fn update_resource_group(params: OperResourceGroupParams) -> CmdResponse<bool> {
    info!("update resource group: {:?}", params);
    let config = ResourceConfig::update_resource_group(params).await;
    CmdResponse::from(config)
}


// 导入资源
#[command]
pub async fn import_resource(params: UploadParams) -> CmdResponse<bool> {
    info!("import resource group");
    let config = ResourceConfig::import_resources(params).await;
    CmdResponse::from(config)
}


use serde::Serialize;

#[derive(Serialize)]
pub struct FontMeta {
    postscript_name : String,
    family: String,
    full_name: String,

}

#[command]
pub fn parse_font_metadata(path: String) -> Result<FontMeta, String> {
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    let face = font_kit::handle::Handle::from_memory(data.into(), 0);
    Ok(FontMeta {
        postscript_name: face.load().unwrap().postscript_name().unwrap_or_default(),
        family: face.load().unwrap().family_name(),
        full_name: face.load().unwrap().full_name(),
    })
}