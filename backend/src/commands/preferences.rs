use crate::{
    models::{config::Config, preferences::Preferences},
    storage::get_config_path,
};
use font_kit::source::SystemSource;
use log;
use tauri::{command, AppHandle, Runtime, process::current_binary, Manager};
use tauri_plugin_opener::OpenerExt;

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
pub fn get_preferences() -> Result<Preferences, String> {
    let config = Config::global();
    Ok(config.preferences().clone())
}

// 更新系统配置
#[command]
pub fn set_preferences(preferences: Preferences) -> Result<(), String> {
    let config = Config::global();
    let mut new_prefs = config.preferences().clone();
    let _ = new_prefs.set_preferences(preferences);
    log::info!("Preferences updated successfully");
    Ok(())
}

// 重置系统配置
#[command]
pub fn restore_preferences() -> Result<(), String> {
    let config = Config::global();
    let mut new_prefs = config.preferences().clone();
    new_prefs.set_preferences(Preferences::default());
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

#[command]
pub async fn restart_app(app: AppHandle){
  app.restart();
}

#[command]
pub fn restart_application<R: Runtime>(app_handle: AppHandle<R>) {
  let env = app_handle.env();
  let path = current_binary(&env).unwrap();
  let arg = std::env::args().collect::<Vec<String>>();
  let mut args = vec!["launch".to_string(), "--".to_string()];
  // filter out the first arg
  if arg.len() > 1 {
      args.extend(arg.iter().skip(1).cloned());
  }
  log::info!("restart app: {:#?} with args: {:#?}", path, args);
  std::process::Command::new(path)
      .args(args)
      .spawn()
      .expect("application failed to start");
  app_handle.exit(0);
  std::process::exit(0);
}