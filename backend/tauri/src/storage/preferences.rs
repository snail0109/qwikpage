use anyhow::Error;
use log::error;
use std::path::PathBuf;

use crate::{
    types::preferences::Preferences,
    utils::{
        dirs::{app_preferences_path, get_default_code_path},
        file::{read_json_file, write_json_file},
    },
};

use super::config::Config;

const DEFAULT_FONT_SIZE: u32 = 12;
const DEFAULT_FONT_BOLD: &str = "normal";

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
            error!("加载应用配置失败: {}", e);
            Self::default()
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = app_preferences_path();
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
