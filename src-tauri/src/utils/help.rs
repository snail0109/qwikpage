use chrono::Local;

use super::constans::DATA_FORMAT;

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
