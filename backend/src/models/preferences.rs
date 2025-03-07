use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::{commands::preferences, storage::{get_config_path, get_default_code_path}};

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
    pub fn new() -> Preferences {
        let path = get_config_path().join("preferences.json");
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei Mono".to_string()
        };
        let mut preferences = Preferences {
            theme: "auto".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            font_family: font_family,
            check_update: false,
            project_path: get_default_code_path(),
        };
        
        match fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(prefs) => {
                    preferences = prefs;
                }
                Err(error) => {
                    println!("Error while parsing file: {:?}\n", error);
                    error!("Error while parsing file: {:?}\n", error);
                }
            },
            Err(error) => {
                preferences.save();
                println!("Error while reading file: {:?}\n", error);
                error!("Error while reading file: {:?}\n", error);
            }
        }
        return preferences;
    }

    pub fn get_preferences(&self) -> Result<Preferences, String>{
        info!("get_preferences");
        // 直接返回当前内存中的状态
        Ok(self.clone())
    }


    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_config_path().join("preferences.json");
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path.clone(), contents.as_bytes())?;
        info!("Preferences saved successfully to {:?}", path);
        Ok(())
    }

    pub fn set_preferences(&mut self, preferences: Preferences) -> Result<(), Error> {
        self.check_update = preferences.check_update;
        self.theme = preferences.theme;
        self.language = preferences.language;
        self.font_size = preferences.font_size;
        self.font_bold = preferences.font_bold;
        self.font_family = preferences.font_family;
        self.project_path = preferences.project_path;
        self.save();
        Ok(())
    }
}

