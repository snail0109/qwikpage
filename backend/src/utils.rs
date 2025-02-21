use chrono::Local;
use std::{net::TcpStream, path::PathBuf};
use dirs;

use crate::constans::{APP_IDENTIFIER, DATA_FORMAT};

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

// 获取应用根目录
pub fn get_app_root_dir() -> PathBuf {
    let root_dir: PathBuf = dirs::data_dir().unwrap().join(APP_IDENTIFIER);
    if !root_dir.exists() {
        std::fs::create_dir_all(&root_dir).unwrap();
    }
    root_dir
}