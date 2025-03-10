use std::fs;

use crate::storage::{get_app_data_path, get_app_root_resource_dir, get_config_path};
use anyhow::Result;

#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用根目录
    get_config_path();

    // 初始化 resources 目录
    get_app_root_resource_dir();

    let prj_path = get_app_data_path().join("code");
    if !prj_path.exists() {
        fs::create_dir_all(&prj_path).unwrap();
    }

    let build_path = get_app_data_path().join("build");

    if !build_path.exists() {
        fs::create_dir_all(&build_path).unwrap();
    }

    // 数据备份迁移

    Ok(())
}
