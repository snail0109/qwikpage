#[macro_use] extern crate rocket;

use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod models;
mod utils;
mod services;
use crate::commands::{dsl, menu, page, project};
use crate::services::demo;
use utils::setup;

use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

pub fn run() {
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init());

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
            dsl::export_json,
        ])
        .setup(|app| {
            setup::init(app);
            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                let _rocket = rocket::build()
                    .mount("/", FileServer::from(relative!("../dist/editor")))
                    .mount("/hello", routes![demo::hello])
                    .launch()
                    .await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}
