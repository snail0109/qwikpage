
mod commands;
mod constans;
mod core;
mod models;
mod service;
mod setup;
mod types;
mod utils;
use crate::{
    commands::{config, dsl, group, page, project},
    service::configure_rocket,
    utils::is_port_in_use,
};
use commands::resource;
use log::{error, info};
use tauri_plugin_log::{Target, TargetKind};
use utils::get_app_root_dir;


const APP_ERROR_MSG: &str = "error while running qwikpage application";

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: get_app_root_dir(),
                        file_name: None,
                    }),
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
            let port = 8789;
            if is_port_in_use(port) {
                error!("Port {} is already in use", port);
            } else {
                tauri::async_runtime::spawn(async move {
                    let rocket = configure_rocket(handle);
                    let _ = rocket.launch().await;
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 分组
            group::add_group,
            group::edit_group,
            group::delete_group,
            group::load_groups,
            group::load_groups_with_projects,
            // 项目
            project::get_project_list,
            project::add_project,
            project::get_project_detail,
            project::update_project,
            project::delete_project,
            // reource
            resource::load_resource,
            resource::add_resource_group,
            resource::delete_resource_group,
            resource::update_resource_group,
            resource::import_resource,
            resource::parse_font_metadata,
            // 页面
            page::get_page_list,
            page::get_page_detail_with_id,
            page::get_page_detail_with_path,
            page::add_page,
            page::update_page,
            page::delete_page,
            page::copy_page,
            // 出码
            dsl::export_json,
            dsl::export_project,
            // 系统配置
            config::open_folder,
            config::set_theme,
            config::get_app_conf,
        ])
        .run(tauri::generate_context!())
        .expect(APP_ERROR_MSG);
}
