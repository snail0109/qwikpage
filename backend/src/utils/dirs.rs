use dirs;
use log::info;
use tauri::{AppHandle, Runtime};
use std::{
    fs,
    path::PathBuf,
};
use tauri_plugin_opener::OpenerExt;

// 应用配置文件夹
const APP_IDENTIFIER: &str = "com.qwikpage.desktop";

// 应用配置数据文件
const APP_SETTING_FILE_NAME: &str = "preferences.json";

// 项目数据存储文件夹
pub const DATA_ROOT_DIR: &str = "QwikPage";

// 返回应用配置目录
pub fn get_config_path() -> PathBuf {
    let config_path: PathBuf = dirs::config_dir().unwrap().join(APP_IDENTIFIER);
    ensure_dir_exists(&config_path);
    config_path
}

// 返回应用配置文件路径
pub fn app_preferences_path() -> PathBuf {
    get_config_path().join(APP_SETTING_FILE_NAME)
}

// 返回DSL数据存储默认目录
pub fn get_app_data_path() -> PathBuf {
    match dirs::preference_dir() {
        Some(path) => {
            let path = path.join(DATA_ROOT_DIR);
            if !path.exists() {
                fs::create_dir_all(&path).unwrap();
            }
            path
        }
        None => get_config_path(),
    }
}

// 返回默认项目DSL数据存储路径
pub fn get_default_code_path() -> String {
    get_app_data_path()
        .join("code")
        .to_string_lossy()
        .to_string()
}

// 返回默认 DSL 出码存放默认路径
pub fn get_default_build_path() -> String {
    get_app_data_path()
        .join("build")
        .to_string_lossy()
        .to_string()
}

// 确保目录存在
pub fn ensure_dir_exists(path: &PathBuf) {
    if !path.exists() {
        info!("create dir: {:?}", path);
        fs::create_dir_all(path).unwrap();
    }
}

// 打开指定路径的文件夹
pub fn open_path<R: Runtime>(app: AppHandle<R>, path: &PathBuf) -> Result<(), String> {
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;
    Ok(())
}
