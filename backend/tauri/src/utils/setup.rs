use std::fs;

use crate::utils::dirs::{ensure_dir_exists, get_app_data_path, get_config_path};
use anyhow::Result;

use super::dirs::{init_preference, projects_group_path};

// 初始化应用文件夹和文件
#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用根目录
    get_config_path();

    init_preference();

    ensure_dir_exists(&get_app_data_path().join("code"));

    ensure_dir_exists(&get_app_data_path().join("build"));

    // 数据备份迁移
    // 如果 group.json 存在 修改文件名称为 projects.json
    let path = get_config_path().join("group.json");
    if path.exists() {
        let new_path = projects_group_path();
        fs::rename(path, new_path).unwrap();
    }

    Ok(())
}
