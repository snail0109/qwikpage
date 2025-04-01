use std::path::PathBuf;

use crate::utils;
use font_kit::source::SystemSource;
use log;
use tauri::{command, AppHandle, Runtime};

// 打开指定路径的文件夹
#[command]
pub fn open_target_folder<R: Runtime>(app: AppHandle<R>, path: &str) -> Result<(), String> {
    log::info!("打开指定路径的文件夹({})", path);
    let target_path = PathBuf::from(path);

    // 检查路径是否存在
    if !target_path.exists() {
        return Err(format!("文件夹({})不存在", path));
    }

    return  utils::dirs::open_path(app, &target_path);

}

// 获取系统字体信息
#[command]
pub fn get_system_fonts() -> Result<Vec<String>, String> {
    log::debug!("SystemService::get_system_fonts(): 获取系统字体信息");
    let source = SystemSource::new();
    Ok(source.all_families().map_err(|e| e.to_string())?)
}


// 重启应用
#[command]
pub fn restart_application<R: Runtime>(app_handle: AppHandle<R>) {
    log::info!("重启应用");
    utils::restart_application(app_handle);
}

// 打开配置目录
#[command]
pub fn open_preferences<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::info!("打开系统配置目录");
    let root_dir = utils::dirs::get_config_path();
    return utils::dirs::open_path(app, &root_dir);
}
