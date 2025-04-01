use crate::{storage::config::Config, types::preferences::Preferences, utils::dirs::app_preferences_path};
use log;
use tauri::command;

// 加载系统配置
#[command]
pub fn get_preferences() -> Result<Preferences, String> {
    log::debug!("TPreferenceService::get_preferences(): 查询系统配置({:?})", app_preferences_path());
    let pref: Preferences = Config::global().preferences().clone();
    Ok(pref)
}

// 更新系统配置
#[command]
pub fn set_preferences(preferences: Preferences) -> Result<(), String> {
    log::debug!("TPreferenceService::get_preferences(): 更新系统配置,项目路径({})" , preferences.project_path.clone());
    let mut new_prefs = Config::global().preferences().clone();
    let _ = new_prefs.set_preferences(preferences);
    Ok(())
}

// 重置系统配置
#[command]
pub fn restore_preferences() -> Result<(), String> {
    log::info!("重置系统配置为默认值");
    let mut new_prefs = Config::global().preferences().clone();
    new_prefs
        .set_preferences(Preferences::default())
        .map_err(|e| format!("重置系统配置失败: {}", e))?;
    Ok(())
}
