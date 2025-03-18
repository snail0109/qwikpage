use std::time::SystemTime;
use chrono::{DateTime, Local};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// 获取当前时间
pub fn get_current_time() -> String {
    Local::now().format(DATE_FORMAT).to_string()
}

// 格式化时间
pub fn format_system_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y/%m/%d").to_string()
}
