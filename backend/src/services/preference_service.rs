use crate::models::{config::Config, preferences::Preferences};
use log;
use tauri::command;

// 加载系统配置
#[command]
pub fn get_preferences() -> Result<Preferences, String> {
    log::debug!("get_preferences");
    let config = Config::global();
    Ok(config.preferences().clone())
}

// 更新系统配置
#[command]
pub fn set_preferences(preferences: Preferences) -> Result<(), String> {
    log::debug!("set_preferences");
    let config = Config::global();
    let mut new_prefs = config.preferences().clone();
    let _ = new_prefs.set_preferences(preferences);
    log::info!("Preferences updated successfully");
    Ok(())
}

// 重置系统配置
#[command]
pub fn restore_preferences() -> Result<(), String> {
    log::debug!("restore_preferences");
    let config = Config::global();
    let mut new_prefs = config.preferences().clone();
    new_prefs
        .set_preferences(Preferences::default())
        .map_err(|e| format!("Failed to restore preferences: {}", e))?;
    log::info!("Preferences updated successfully");
    Ok(())
}
