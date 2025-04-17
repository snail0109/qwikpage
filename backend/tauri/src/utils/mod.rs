pub mod datetime;
pub mod dirs;
pub mod file;
pub mod setup;

use chrono::Local;
use log::{info, Level};
use std::{net::TcpStream, path::Path};

use tauri::{process::current_binary, AppHandle, Manager, Runtime};

// 分页
pub fn paginate<T: Clone>(items: Vec<T>, page_num: usize, page_size: usize) -> (Vec<T>, usize) {
    let start = (page_num - 1) * page_size;
    let end = start + page_size;
    let end = end.min(items.len());
    let total = items.len();
    (items[start..end].to_vec(), total)
}
// 检查端口是否被占用
pub fn check_port_occupied(port: u16) -> bool {
    let address = format!("127.0.0.1:{}", port);
    TcpStream::connect(address).is_ok()
}

pub fn restart_application<R: Runtime>(app_handle: AppHandle<R>) {
    let env = app_handle.env();
    let path = current_binary(&env).unwrap();
    let arg = std::env::args().collect::<Vec<String>>();
    let mut args = vec!["launch".to_string(), "--".to_string()];
    // filter out the first arg
    if arg.len() > 1 {
        args.extend(arg.iter().skip(1).cloned());
    }
    info!("restart app: {:#?} with args: {:#?}", path, args);
    std::process::Command::new(path)
        .args(args)
        .spawn()
        .expect("application failed to start");
    app_handle.exit(0);
    std::process::exit(0);
}

pub fn custom_log_out(
    out: tauri_plugin_log::fern::FormatCallback<'_>,
    message: &std::fmt::Arguments<'_>,
    record: &log::Record<'_>,
) {
    // 自定义日志级别映射
    let level_short = match record.level() {
        Level::Error => "E", // Error -> E
        Level::Warn => "W",  // Warn -> W
        Level::Info => "I",  // Info -> I
        Level::Debug => "D", // Debug -> D
        Level::Trace => "T", // Trace -> T
    };
    // 区分windows 和 mac
    let file_path = record.file_static().unwrap_or("<unknown>"); // 使用默认值避免panic
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned()) // 将文件名转换为字符串
        .unwrap_or_else(|| {
            // 如果无法获取文件名，使用整个路径或默认值
            if file_path.is_empty() {
                "<unknown>".to_string()
            } else {
                file_path.to_string()
            }
        });

    out.finish(format_args!(
        "{} ({}:{:#?}) [{}] > {}",
        Local::now().format("%H:%M:%S").to_string(),
        file_name,
        record.line().unwrap_or(0),
        level_short,
        message
    ))
}
