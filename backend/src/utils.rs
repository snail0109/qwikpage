use chrono::{DateTime, Local};
use std::{
    net::TcpStream,
    path::PathBuf,
    time::SystemTime,
};

use crate::constans::DATA_FORMAT;

// 获取当前时间
pub fn get_current_time() -> String {
    Local::now().format(DATA_FORMAT).to_string()
}

// 分页
pub fn paginate<T: Clone>(items: Vec<T>, page_num: usize, page_size: usize) -> (Vec<T>, usize) {
    let start = (page_num - 1) * page_size;
    let end = start + page_size;
    let end = end.min(items.len());
    let total = items.len();
    (items[start..end].to_vec(), total)
}

// 检查端口是否被占用
pub fn is_port_in_use(port: u16) -> bool {
    let address = format!("127.0.0.1:{}", port);
    TcpStream::connect(address).is_ok()
}


// 格式化时间
pub fn format_system_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y/%m/%d").to_string()
}

// 格式化文件大小
pub fn format_system_size(size: u64) -> String {
    #[cfg(target_os = "macos")]
    let use_base_10 = true; // macOS 使用 base-10

    #[cfg(not(target_os = "macos"))]
    let use_base_10 = false; // 其他系统 (如 Windows) 使用 base-2

    if use_base_10 {
        // Base-10 (decimal)
        if size < 1000 {
            format!("{} B", (size as f64 / 1000.0).round() as u64)
        } else if size < 1000 * 1000 {
            format!("{} KB", (size as f64 / 1000.0).round() as u64)
        } else if size < 1000 * 1000 * 1000 {
            format!("{:.1} MB", size as f64 / (1000.0 * 1000.0))
        } else {
            format!("{:.2} GB", size as f64 / (1000.0 * 1000.0 * 1000.0))
        }
    } else {
        // Base-2 (binary)
        if size < 1024 {
            format!("{} B", (size as f64 / 1024.0).round() as u64)
        } else if size < 1024 * 1024 {
            format!("{} KB", (size as f64 / 1024.0).round() as u64)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

// 判断 path 是有效文件，忽略隐藏文件
pub fn is_valid_file(path: &PathBuf) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.starts_with(".") {
        return false;
    }
    path.is_file()
}
