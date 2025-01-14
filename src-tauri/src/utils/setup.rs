use crate::utils::dirs;
use anyhow::Result;
use log::info;
use std::fs;

use super::constans::PAGE_DIR;

#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    info!("qwikpage app start");
    let app_config_dir = dirs::app_config_dir().unwrap();
    let app_data_dir = dirs::app_data_dir().unwrap();
    if !app_config_dir.exists() {
        info!("create app config dir: {:?}", app_config_dir);
        fs::create_dir_all(&app_config_dir).expect("failed to create app config dir");
    }
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
    Ok(())
}
