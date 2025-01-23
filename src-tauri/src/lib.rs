mod commands;
mod core;
mod models;
mod services;
mod utils;
use crate::{
    commands::{config, dsl, menu, page, project},
    core::setup,
    services::preview,
};
use log::{info, error};
use tauri_plugin_log::{Target, TargetKind};
use utils::help::is_port_in_use;

const APP_ERROR_MSG: &str = "error while running qwikpage application";

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
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
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            info!("============== Start App ==============");
            setup::init(app)?;
            let handle = app.handle().clone();
            // mount the rocket instance
            let port = 8000;
            if is_port_in_use(port) {
                error!("Port {} is already in use", port);
            } else {
                info!("Port {} ", port);
                tauri::async_runtime::spawn(async move {
                    let rocket = preview::configure_rocket(handle);
                    let _ = rocket.launch().await;
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            project::get_project_list,
            project::add_project,
            project::get_project_detail,
            project::update_project,
            project::delete_project,
            menu::get_menu_list,
            menu::add_menu,
            menu::copy_menu,
            menu::update_menu,
            menu::delete_menu,
            page::get_page_list,
            page::get_page_detail,
            page::add_page,
            page::update_page,
            page::delete_page,
            page::copy_page,
            dsl::export_json,
            config::open_folder,
            config::set_theme,
            config::get_app_conf,
        ])
        .run(tauri::generate_context!())
        .expect(APP_ERROR_MSG);
}
