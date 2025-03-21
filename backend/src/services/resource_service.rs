use log;
use tauri::command;

use crate::{
    storage::resource::{
        DeleteResource, OperResourceGroupParams, RenameResource, ResourceConfig, ResourceGroupInfo,
        ResourceQueryParams, UploadParams,
    },
    types::{js_resp::JSResp, resource::FontMeta},
    utils::file::load_font_metadata,
};

// 查询分组资源信息
#[command]
pub async fn load_resource(params: ResourceQueryParams) -> JSResp<Vec<ResourceGroupInfo>> {
    log::debug!("load resource: {:?}", params);
    let config = ResourceConfig::load(params).await;
    JSResp::from(config)
}

// 创建资源分组(目录)
#[command]
pub async fn add_resource_group(params: OperResourceGroupParams) -> JSResp<bool> {
    log::debug!("add resource group: {:?}", params);
    let config = ResourceConfig::add_resource_group(params).await;
    JSResp::from(config)
}

// 删除资源分组(目录)
#[command]
pub async fn delete_resource_group(params: OperResourceGroupParams) -> JSResp<bool> {
    log::debug!("delete resource group: {:?}", params);
    let config = ResourceConfig::delete_resource_group(params).await;
    JSResp::from(config)
}

// 更新资源分组(目录)
#[command]
pub async fn update_resource_group(params: OperResourceGroupParams) -> JSResp<bool> {
    log::debug!("update resource group: {:?}", params);
    let config = ResourceConfig::update_resource_group(params).await;
    JSResp::from(config)
}

// 导入资源
#[command]
pub async fn import_resource(params: UploadParams) -> JSResp<bool> {
    log::debug!("import resource group");
    let config = ResourceConfig::import_resources(params).await;
    JSResp::from(config)
}

// 重命名资源
#[command]
pub async fn rename_resource(params: RenameResource) -> JSResp<bool> {
    log::debug!("rename resource: {:?}", params);
    let config = ResourceConfig::rename_resource(params).await;
    JSResp::from(config)
}

// 删除资源
#[command]
pub async fn delete_resource(params: DeleteResource) -> JSResp<bool> {
    log::debug!("delete resource: {:?}", params);
    let config = ResourceConfig::delete_resource(params).await;
    JSResp::from(config)
}

// 解析字体元数据
// 参数: path - 字体文件的路径
#[command]
pub fn parse_font_metadata(path: String) -> JSResp<FontMeta> {
    log::debug!("parse font metadata: {:?}", path);
    JSResp::from(load_font_metadata(path))
}
