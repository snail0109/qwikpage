use std::path::PathBuf;

use tauri::{command, AppHandle, Manager, Runtime};
use tauri_plugin_opener::OpenerExt;

#[command]
pub fn open_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    
    let config_dir: PathBuf = app.path().config_dir().unwrap();
    let root_dir: PathBuf = config_dir.join("qwikpage");
    app.opener()
        .open_path(root_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;
    Ok(())
}
