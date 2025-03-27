mod code_generator;
mod error;
mod services;
mod storage;
mod types;
mod utils;

use crate::{
    services::{
        code_service, group_service, page_service, preference_service, preview_service,
        project_service, resource_service, sys_service,
    },
    utils::{check_port_occupied, dirs::get_config_path, setup},
};
use code_core::hello;
use log::{self, Level};
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
    hello();
    tauri::Builder::default()
        // 单实例插件确保 Tauri 应用程序在同一时间只运行单个实例
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(
            // 日志插件
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: get_config_path(),
                        file_name: None,
                    }),
                    Target::new(TargetKind::Webview),
                ])
                .level(if is_dev() {
                    log::LevelFilter::Trace
                } else {
                    log::LevelFilter::Info
                })
                .format(move |out, message, record| {
                
                    // 定义颜色（ANSI 转义码）
                    let level_color = match record.level() {
                        Level::Error => "\x1b[31m", // 红色
                        Level::Warn => "\x1b[33m",  // 黄色
                        Level::Info => "\x1b[34m",  // 蓝色
                        Level::Debug => "\x1b[32m", // 绿色
                        Level::Trace => "\x1b[36m", // 青色
                        };
                        let target_color = "\x1b[35m"; // 紫色
                        let message_color = match record.level() {
                            Level::Error => "\x1b[31m",
                            Level::Warn => "\x1b[33m",
                            _ => "\x1b[0m", // 默认无颜色
                        };
                        out.finish(format_args!(
                            "{level_color}[{level}]{reset} {target_color}[{target}]{reset} > {message_color}{message}\x1b[0m",
                            level = record.level(),
                            target = record.target(),
                            message = message,
                            level_color = level_color,
                            target_color = target_color,
                            message_color = message_color,
                            reset = "\x1b[0m"
                        ))
                        
                           
                })
                .max_file_size(50000)
                // .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            log::trace!("Start QwikPage Application");
            // 全局 AppHandle 实例
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
            log::trace!("Init App Config Store");
            setup::init(app)?;

            log::trace!("Init Preview Service");
            // 初始化 rocket 服务，用于项目页面预览
            let handle = app.handle().clone();
            let port = 8789;
            if check_port_occupied(port) {
                log::error!("Port {} is already in use", port);
            } else {
                tauri::async_runtime::spawn(async move {
                    let rocket = preview_service::configure_rocket(handle);
                    let _ = rocket.launch().await;
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 分组
            group_service::add_group,
            group_service::edit_group,
            group_service::delete_group,
            group_service::load_groups,
            group_service::load_groups_with_projects,
            // 项目
            project_service::get_project_list,
            project_service::add_project,
            project_service::get_project_detail,
            project_service::update_project,
            project_service::delete_project,
            project_service::upload_project_resource,
            // 资源管理
            resource_service::load_resource,
            resource_service::add_resource_group,
            resource_service::delete_resource_group,
            resource_service::update_resource_group,
            resource_service::import_resource,
            resource_service::rename_resource,
            resource_service::delete_resource,
            resource_service::parse_font_metadata,
            // 页面
            page_service::get_page_list,
            page_service::get_page_detail_with_id,
            page_service::get_page_detail_with_path,
            page_service::add_page,
            page_service::update_page,
            page_service::delete_page,
            page_service::copy_page,
            // 出码
            code_service::export_json,
            code_service::export_project,
            // 配置服务
            preference_service::get_preferences,
            preference_service::set_preferences,
            preference_service::restore_preferences,
            // 系统服务
            sys_service::open_target_folder,
            sys_service::get_system_fonts,
            sys_service::open_preferences,
            sys_service::restart_application,
        ])
        .run(tauri::generate_context!())
        .expect("error while running qwikpage application");
}

fn is_dev() -> bool {
    #[cfg(dev)]
    {
        return true;
    }
    #[cfg(not(dev))]
    {
        return false;
    }
}
