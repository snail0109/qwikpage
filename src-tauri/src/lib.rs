use tauri_plugin_log::{Target, TargetKind};

mod cmd;
mod types;
use cmd::{ menu, page, project };

mod utils;

use crate::utils::init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化系配置文件
    // init::init_config();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            project::get_project_list,
            project::get_project_detail,
            project::add_project,
            project::update_project,
            project::delete_project,
            menu::get_menu_list,
            menu::add_menu,
            menu::copy_menu,
            menu::update_menu,
            menu::delete_menu,
            page::get_page_list,
            page::get_page_detail,
            page::create_page,
            page::update_page,
            page::delete_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}
