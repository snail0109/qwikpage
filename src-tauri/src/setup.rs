use crate::{core::conf::AppConf, utils::get_app_root_dir};
use anyhow::Result;
use log::info;
use std::fs;

use crate::constans::PAGE_DIR;

#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let conf = &AppConf::load(handle)?;
    
    let app_data_dir = get_app_root_dir();
    
    if !app_data_dir.exists() {
        info!("create app data dir: {:?}", app_data_dir);
        fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
    }

    // 初始化 page 目录
    let page_dir = app_data_dir.join(PAGE_DIR);
    if !page_dir.exists() {
        info!("create page_dir dir: {:?}", page_dir);
        fs::create_dir_all(&page_dir).expect("failed to create pages dir");
    }

    // 数据备份迁移

    Ok(())
}
