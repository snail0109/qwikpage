use anyhow::{Context, Error, Result};
use dirs;
use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use crate::constans::APP_IDENTIFIER;

// 返回全局应用配置
pub fn get_config_path() -> PathBuf {
    let config_path: PathBuf = dirs::config_dir().unwrap().join(APP_IDENTIFIER);
    if !config_path.exists() {
        info!("create root dir: {:?}", config_path);
        create_dir_all(&config_path).unwrap();
    }
    config_path
}

// 返回应用资源目录
pub fn get_app_root_resource_dir() -> PathBuf {
    let config_path = get_config_path();
    let resources_path = config_path.join("resources");
    if !resources_path.exists() {
        info!("create resources dir: {:?}", resources_path);
        create_dir_all(&resources_path).expect("failed to create resources dir");
    }
    resources_path
}

// 返回全局数据存储目录
pub fn get_app_data_path() -> PathBuf {
    match dirs::preference_dir() {
        Some(path) => {
            let path = path.join("QwikPage");
            if !path.exists() {
                fs::create_dir_all(&path).unwrap();
            }
            path
        }
        None => get_config_path(),
    }
}

pub fn get_default_code_path() -> String {
    get_app_data_path()
        .join("code")
        .to_string_lossy()
        .to_string()
}

pub fn get_default_build_path() -> String {
    get_app_data_path()
        .join("build")
        .to_string_lossy()
        .to_string()
}

pub fn app_preferences_path() -> PathBuf {
    get_config_path().join("preferences.json")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalStorage {
    conf_path: PathBuf,
}

impl LocalStorage {
    pub fn new(file_name: String) -> Self {
        // 判断系统是否存在 file_name 的文件
        let config_path = get_config_path();
        Self {
            conf_path: config_path.join(file_name),
        }
    }

    // pub fn load(&self) -> String {
    //     let contents = fs::read_to_string(self.conf_path)?;
    //     contents
    // }
}

pub fn write_json_file<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Error> {
    let json_str = serde_json::to_string_pretty(data)?;
    let path_str = path.as_os_str().to_string_lossy().to_string();
    fs::write(path, json_str.as_bytes())
        .with_context(|| format!("failed to save file \"{path_str}\""))
}

pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let contents = fs::read_to_string(path)?;
    let value = serde_json::from_str(&contents)?;
    Ok(value)
}
