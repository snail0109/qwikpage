use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
};
use tauri::{AppHandle, Manager, Theme};

use crate::utils::constans::APP_NAME;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConf {
    pub theme: String,
}
// 应用级别配置
impl AppConf {
    pub fn new() -> Self {
        Self {
            theme: "system".to_string(),
        }
    }

    pub fn get_conf_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = app
            .path()
            .config_dir()?
            .join(APP_NAME)
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

    pub fn get_theme(app: &AppHandle) -> Theme {
        let theme = Self::load(app).unwrap().theme;
        match theme.as_str() {
            "system" => match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
                dark_light::Mode::Default => Theme::Light,
            },
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}
