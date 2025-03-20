use dirs;
use log::info;
use std::{
    fs,
    path::PathBuf,
};

// 应用配置文件夹
const APP_IDENTIFIER: &str = "com.qwikpage.desktop";

const APP_SETTING_FILE_NAME: &str = "preferences.json";

// 页面存储文件夹
pub const PAGE_DIR: &str = "pages";

// 项目数据存储文件夹
pub const DATA_ROOT_DIR: &str = "QwikPage";

// 返回全局配置目录
pub fn get_config_path() -> PathBuf {
    let config_path: PathBuf = dirs::config_dir().unwrap().join(APP_IDENTIFIER);
    ensure_dir_exists(&config_path);
    config_path
}


// 返回数据存储默认目录
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

// 获取默认项目数据存储路径
pub fn get_default_code_path() -> String {
    get_app_data_path()
        .join("code")
        .to_string_lossy()
        .to_string()
}

// 获取默认出码路径
pub fn get_default_build_path() -> String {
    get_app_data_path()
        .join("build")
        .to_string_lossy()
        .to_string()
}

// 返回全局配置文件路径
pub fn app_preferences_path() -> PathBuf {
    get_config_path().join(APP_SETTING_FILE_NAME)
}

// 检查文件是否存在
pub fn ensure_dir_exists(path: &PathBuf) {
    if !path.exists() {
        info!("create dir: {:?}", path);
        fs::create_dir_all(path).unwrap();
    }
}


