use crate::{manager::preference_manager::PreferencesManager, types::preferences::Preferences, utils::dirs::app_preferences_path};
use log;
use tauri::command;

// 加载系统配置
#[command]
pub fn get_preferences() -> Result<Preferences, String> {
    log::debug!("TPreferenceService::get_preferences(): 查询系统配置({:?})", app_preferences_path());
    let pref: Preferences = PreferencesManager::get_preferences().clone();
    log::info!("TPreferenceService::get_preferences(): 查询系统配置成功,默认项目({:?})", pref.project_path);
    Ok(pref)
}

// 更新系统配置
#[command]
pub fn set_preferences(preferences: Preferences) -> Result<(), String> {
    log::debug!("TPreferenceService::get_preferences(): 更新系统配置,项目路径({})" , preferences.project_path.clone());
    match PreferencesManager::set_preferences(preferences) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// 重置系统配置
#[command]
pub fn restore_preferences() -> Result<(), String> {
    log::info!("重置系统配置为默认值");
    match PreferencesManager::set_preferences(Preferences::default()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
