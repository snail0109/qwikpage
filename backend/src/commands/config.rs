use tauri::{command, AppHandle, Runtime};
use tauri_plugin_opener::OpenerExt;
use crate::{models::conf::AppConf, utils::get_app_root_dir};

#[command]
pub fn open_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    
    let root_dir = get_app_root_dir();
    app.opener()
        .open_path(root_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;
    Ok(())
}


#[command]
pub fn set_theme(app: AppHandle, theme: String) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"theme": theme}))
        .unwrap()
        .save(&app)
        .unwrap();

    // app.restart();
}

#[command]
pub fn get_app_conf(app: AppHandle) -> AppConf {
    AppConf::load(&app).unwrap()
}