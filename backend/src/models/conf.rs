use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
};
use tauri::{AppHandle, Manager};

use crate::utils::get_store_path;


const DEFAULT_FONT_SIZE : u32 = 12;
const DEFAULT_FONT_BOLD : &str  = "normal";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConf {
    pub theme: String,         // 主题
    pub language: String,      // 预研
    pub font_size: u32,     // 字体大小
    pub font_bold: String,     // 是否粗体
    pub font_family: String,   // 字体
    pub check_update: bool,    // 自动更新
    pub dsl_code_dir: String   // DSL代码目录
}

// 应用级别配置
impl AppConf {
    pub fn new() -> Self {
        // mac 和 windows 使用不同的默认字体
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei".to_string()
        };
        Self {
            theme: "system".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            font_family: font_family,
            check_update: false,
            dsl_code_dir: get_store_path().join("code").to_string_lossy().to_string()
        }
    }

    pub fn get_conf_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = app
            .path()
            .app_config_dir()?
            .join("config.json");
        Ok(config_dir)
    }


    pub fn load(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_conf_path(app)?;

        if !path.exists() {
            let config = Self::new();
            config.save(app)?;
            return Ok(config);
        } 

        // a convenience function for using [`File::open`] and [`read_to_string`]
        let contents = fs::read_to_string(path)?;
        let config: Result<AppConf, _> = serde_json::from_str(&contents);

        // Handle conditional fields and fallback to defaults if necessary
        if let Err(e) = &config {
            error!("[conf::load] {}", e);
            let mut default_config = Self::new();
            default_config = default_config.amend(serde_json::from_str(&contents)?)?;
            default_config.save(app)?;
            return Ok(default_config);
        }

        Ok(config?)
    }

    pub fn save(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_conf_path(app)?;

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        // a convenience function for using [`File::create`] and [`write_all`]
        fs::write(path, contents.as_bytes())?;
        Ok(())
    }

    pub fn amend(self, json: Value) -> Result<Self, serde_json::Error> {
        let val = serde_json::to_value(self)?;
        let mut config: BTreeMap<String, Value> = serde_json::from_value(val)?;
        let new_json: BTreeMap<String, Value> = serde_json::from_value(json)?;

        for (k, v) in new_json {
            config.insert(k, v);
        }

        let config_str = serde_json::to_string_pretty(&config)?;
        serde_json::from_str::<AppConf>(&config_str).map_err(|err| {
            error!("[conf::amend] {}", err);
            err
        })
    }

}
