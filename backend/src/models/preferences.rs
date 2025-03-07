use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::storage::{get_config_path, get_default_code_path};

const DEFAULT_FONT_SIZE: u32 = 12;
const DEFAULT_FONT_BOLD: &str = "normal";

#[derive(Serialize, Deserialize, Clone, Debug)] 
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub theme: String,        // 主题
    pub language: String,     // 预研
    pub font_size: u32,       // 字体大小
    pub font_bold: String,    // 是否粗体
    pub font_family: String,  // 字体
    pub check_update: bool,   // 自动更新
    pub project_path: String, // DSL代码目录
}

// 应用级别配置
impl Preferences {
    pub fn new() -> Self {
        // mac 和 windows 使用不同的默认字体
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei Mono".to_string()
        };
        Self {
            theme: "auto".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            font_family: font_family,
            check_update: false,
            project_path: get_default_code_path(),
        }
    }

    pub fn get_preferences_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let preferences_dir = get_config_path().join("preferences.json");
        Ok(preferences_dir)
    }

    pub fn get_preferences() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Preferences::load preferences");
        let path = Self::get_preferences_path()?;

        if !path.exists() {
            let preferences = Self::new();
            preferences.save()?;
            return Ok(preferences);
        }

        let contents = fs::read_to_string(path)?;
        let preferences: Result<Preferences, _> = serde_json::from_str(&contents);

        // Handle conditional fields and fallback to defaults if necessary
        if let Err(e) = &preferences {
            error!("[preferences::load] {}", e);
            let mut preferences = Self::new();
            preferences = preferences.set_preferences(serde_json::from_str(&contents)?)?;
            preferences.save()?;
            return Ok(preferences);
        }

        Ok(preferences?)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_preferences_path()?;

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        // a convenience function for using [`File::create`] and [`write_all`]
        fs::write(path, contents.as_bytes())?;
        Ok(())
    }

    pub fn set_preferences(self, json: Value) -> Result<Self, serde_json::Error> {
        info!("Preferences::set_preferences {}", json);
        let val = serde_json::to_value(self)?;
        let mut preferences: BTreeMap<String, Value> = serde_json::from_value(val)?;
        let new_json: BTreeMap<String, Value> = serde_json::from_value(json)?;

        for (k, v) in new_json {
            preferences.insert(k, v);
        }

        let preferences_str = serde_json::to_string_pretty(&preferences)?;
        serde_json::from_str::<Preferences>(&preferences_str).map_err(|err| {
            error!("[Preferences::set_preferences] {}", err);
            err
        })
    }
}
