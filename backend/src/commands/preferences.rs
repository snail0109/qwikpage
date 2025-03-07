use crate::{models::preferences::{default_preferences, Preferences}, storage::get_config_path, PreferencesState};
use font_kit::source::SystemSource;
use tauri::{command, AppHandle, Runtime};
use tauri_plugin_opener::OpenerExt;
use log;


// 打开指定路径的文件夹
#[command]
pub fn open_target_folder<R: Runtime>(app: AppHandle<R>, path: &str) -> Result<(), String> {
    // 将传入的路径转换为 Path
    let target_path = std::path::Path::new(path);

    // 检查路径是否存在
    if !target_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // 使用 app.opener() 打开指定路径
    app.opener()
        .open_path(target_path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;

    Ok(())
}

// 获取系统字体信息
#[command]
pub fn get_system_fonts() -> Result<Vec<String>, String> {
    let source = SystemSource::new();

    Ok(source.all_families().map_err(|e| e.to_string())?)
}

// 加载系统配置
#[command]
pub fn get_preferences(
    preferences_state: tauri::State<PreferencesState>,
) -> Result<Preferences, String> {
    let preferences_state = preferences_state.0.lock().unwrap();
    preferences_state.get_preferences()
}

// 更新系统配置
#[command]
pub fn set_preferences(
    preferences_state: tauri::State<PreferencesState>,
    preferences: Preferences,
) -> Result<(), String> {
    let mut preferences_state = preferences_state.0.lock().map_err(|e| e.to_string())?;
    preferences_state.set_preferences(preferences).map_err(|e| e.to_string())?;
    log::info!("Preferences updated successfully");
    Ok(())
}

// 重置系统配置
#[command]
pub fn restore_preferences(
    preferences_state: tauri::State<PreferencesState>,
) -> Result<(), String> {
    let mut preferences_state = preferences_state.0.lock().map_err(|e| e.to_string())?;
    preferences_state.set_preferences(default_preferences()).map_err(|e| e.to_string())?;
    log::info!("Preferences updated successfully");
    Ok(())
}


// 打开配置目录
#[command]
pub fn open_preferences<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let root_dir = get_config_path();
    app.opener()
        .open_path(root_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;
    Ok(())
}

