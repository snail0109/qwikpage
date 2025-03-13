mod code_generator;
mod commands;
mod constans;
mod error;
mod models;
mod service;
mod setup;
mod types;
mod utils;
mod storage;

use crate::{
    commands::{code, preferences, group, page, project, resource},
    service::configure_rocket,
    utils::is_port_in_use,
    storage::get_config_path,
};
use log::{error, info};
use once_cell::sync::OnceCell;
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_log::{Target, TargetKind};

// #[cfg(target_os = "windows")]
// use {
//     tauri::Manager,
//     webview2_com::Error as Webview2Error,
//     winreg
// };

const DEFAULT_WINDOW_WIDTH: f64 = 1100.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;

const MIN_WINDOW_WIDTH: f64 = 300.0;
const MIN_WINDOW_HEIGHT: f64 = 300.0;

// Global AppHandle
pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

// #[cfg(target_os = "windows")]
// fn check_webview2_installation() -> Result<(), String> {
//     // 检查注册表中是否存在WebView2 Runtime
//     let hklm = winreg::RegKey::predef(winreg::enums::HKEY::HKEY_LOCAL_MACHINE);
//     let webview2_key = hklm
//         .open_subkey(
//             r"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
//         )
//         .or_else(|_| {
//             hklm.open_subkey(
//                 r"SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
//             )
//         });

//     match webview2_key {
//         Ok(_) => {
//             info!("WebView2 Runtime 已安装。")
//             Ok(())
//         },
//         Err(_) => Err("未检测到 WebView2 Runtime，请先安装 WebView2 Runtime：https://developer.microsoft.com/microsoft-edge/webview2/".to_string()),
//     }
// }

pub fn run() {
    // #[cfg(target_os = "windows")]
    // if let Err(err) = check_webview2_installation() {
    //     error!("{}", err);
    //     std::process::exit(1);
    // }

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: get_config_path(),
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
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            preferences::get_system_fonts,
            preferences::open_target_folder,
            preferences::get_preferences,
            preferences::set_preferences,
            preferences::restore_preferences,
            preferences::open_preferences,
            preferences::restart_app,
            preferences::restart_application,
        ])
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}
