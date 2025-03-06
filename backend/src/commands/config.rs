
use crate::utils::get_app_root_dir;
use tauri::{command, AppHandle, Runtime};
use tauri_plugin_opener::OpenerExt;
use font_kit::source::SystemSource;

#[command]
pub fn open_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let root_dir = get_app_root_dir();
    app.opener()
        .open_path(root_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;
    Ok(())
}

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


// #[command]
// pub fn get_config(state: tauri::State<AppConfState> ) -> AppConf {
//     let state = state.0.lock().unwrap();
//     state.clone()
// }
