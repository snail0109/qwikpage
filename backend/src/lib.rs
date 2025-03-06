mod code_generator;
mod commands;
mod constans;
mod error;
mod models;
mod service;
mod setup;
mod types;
mod utils;

use crate::{
    commands::{code, config, group, page, project, resource},
    service::configure_rocket,
    utils::{get_app_root_dir, is_port_in_use},
};
use log::{error, info};
use once_cell::sync::OnceCell;
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_log::{Target, TargetKind};
use utils::get_app_root_resource_dir;

const DEFAULT_WINDOW_WIDTH: f64 = 1100.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;

const MIN_WINDOW_WIDTH: f64 = 300.0;
const MIN_WINDOW_HEIGHT: f64 = 300.0;

// Global AppHandle
pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

// pub struct AppConfState(pub Mutex<AppConf>);


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
        .plugin(tauri_plugin_store::Builder::default().build())
        // .manage(AppConfState(Mutex::new(AppConf::default())))
        .setup(|app| {
            info!("============== Start App ==============");
            // Global AppHandle
            APP.get_or_init(|| app.handle().clone());
            let mut win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("")
                .resizable(true)
                .fullscreen(false)
                .disable_drag_drop_handler()
                .inner_size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
                .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

            // 仅在 macOS 时设置透明标题栏
            #[cfg(target_os = "macos")]
            {
                win_builder = win_builder
                    .hidden_title(true)
                    .title_bar_style(TitleBarStyle::Overlay);
            }

            // Add non-MacOS things
            #[cfg(not(target_os = "macos"))]
            {
                // Doesn't seem to work from Rust, here, so we do it in main.tsx
                win_builder = win_builder.decorations(false);
            }
            win_builder.build().unwrap();

            // Init Config
            info!("Init App Config Store");
            setup::init(app)?;

            // 初始化 resources 目录
            get_app_root_resource_dir();

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
            project::upload_project_resource,
            // reource
            resource::load_resource,
            resource::add_resource_group,
            resource::delete_resource_group,
            resource::update_resource_group,
            resource::import_resource,
            resource::rename_resource,
            resource::delete_resource,
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
            code::export_json,
            code::export_project,
            // 系统配置
            config::open_folder,
            config::get_system_fonts,
            config::open_target_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}
