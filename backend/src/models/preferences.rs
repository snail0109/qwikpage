use std::path::PathBuf;

use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::utils::{
    dirs::{app_preferences_path, get_default_code_path},
    file::{read_json_file, write_json_file},
};

use super::config::Config;

const DEFAULT_FONT_SIZE: u32 = 12;
const DEFAULT_FONT_BOLD: &str = "normal";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub theme: String,        // 主题
    pub language: String,     // 语言
    pub font_size: u32,       // 字体大小
    pub font_bold: String,    // 是否粗体
    pub font_family: String,  // 字体
    pub check_update: bool,   // 自动更新
    pub project_path: String, // DSL代码目录
}

impl Default for Preferences {
    fn default() -> Self {
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei Mono".to_string()
        };
        Self {
            font_family,
            theme: "auto".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            check_update: true,
            project_path: get_default_code_path(),
        }
    }
}

// 应用级别配置
impl Preferences {
    pub fn new() -> Preferences {
        Self::load()
    }

    pub fn load() -> Preferences {
        let path = app_preferences_path();
        read_json_file(&path).unwrap_or_else(|e| {
            error!("Failed to load preferences: {}", e);
            Self::default()
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = app_preferences_path();
        info!("Save preferences to: {}", path.display());
        write_json_file(&path, self)
    }

    pub fn set_preferences(&mut self, preferences: Preferences) -> Result<(), Error> {
        *self = preferences.clone();
        self.save()?;
        Config::global().update_preferences(|current| {
            *current = preferences;
        });
        Ok(())
    }

    pub fn get_project_path(&self) -> PathBuf {
        PathBuf::from(self.project_path.clone())
    }
}
