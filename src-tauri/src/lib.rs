use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod models;
mod utils;

use crate::commands::{menu, page, project};
use utils::setup;

pub fn run() {
    setup::init().expect("failed to initialize qwikpage app");

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
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
        .plugin(tauri_plugin_opener::init());

    builder
        // .setup(|app|
        //     // /Users/**/Library/Application Support/com.qwikpage.iwhalecloud
        //     let app_data_dir = app.path().app_data_dir().unwrap();
        //     create_dir_all(app_data_dir.clone()).expect("Problem creating App directory!");
        // )
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}
