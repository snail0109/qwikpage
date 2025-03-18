pub mod datetime;
pub mod file;
pub mod dirs;
pub mod setup;

use std::net::TcpStream;

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

