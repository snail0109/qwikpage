use crate::{models::conf::AppConf, utils::{get_app_root_dir, get_app_root_resource_dir}};
use anyhow::Result;

#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let conf = &AppConf::load(handle)?;
    
    // 初始化应用根目录
    get_app_root_dir();
    
    // 初始化 resources 目录
    get_app_root_resource_dir();

    // 数据备份迁移

    Ok(())
}
