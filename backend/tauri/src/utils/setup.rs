use crate::utils::dirs::{ensure_dir_exists, get_app_data_path, get_config_path};
use anyhow::Result;

use super::dirs::init_preference;

// 初始化应用文件夹和文件
#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用根目录
    get_config_path();

    init_preference();

    ensure_dir_exists(&get_app_data_path().join("code"));

    ensure_dir_exists(&get_app_data_path().join("build"));

    // 数据备份迁移

    Ok(())
}
