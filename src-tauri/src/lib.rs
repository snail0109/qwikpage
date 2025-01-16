#[macro_use]
extern crate rocket;

use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod models;
mod services;
mod utils;
use crate::commands::{dsl, menu, page, project};
use crate::services::preview;
use rocket::fs::{relative, FileServer, NamedFile};
use utils::setup;

const APP_ERROR_MSG: &str = "error while running qwikpage application";

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    // 返回 SPA 的入口文件，例如 index.html
    NamedFile::open(relative!("../dist/admin/index.html"))
        .await
        .ok()
}

fn configure_rocket() -> rocket::Rocket<rocket::Build> {
    let static_path = relative!("../dist/admin");
    rocket::build()
        .mount(
            "/api",
            routes![
                preview::get_project_detail,
                preview::get_project_menus,
                preview::get_menu_detail,
                preview::get_page_detail
            ],
        )
        .mount("/", FileServer::from(static_path))
        .register("/", catchers![not_found])
}

pub fn run() {
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
        .plugin(tauri_plugin_dialog::init())
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
            setup::init(app)?;
            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                let _rocket = configure_rocket().launch().await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect(APP_ERROR_MSG);
}
