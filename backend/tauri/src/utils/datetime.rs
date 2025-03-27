use std::time::SystemTime;
use chrono::{DateTime, Local};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

const RESOURCE_DATE_FORMAT: &str = "%Y/%m/%d";

// 获取当前时间
pub fn get_current_time() -> String {
    Local::now().format(DATE_FORMAT).to_string()
}

// 格式化本地文件修改和操作时间，用于资源管理前端界面显示
pub fn format_resource_system_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format(RESOURCE_DATE_FORMAT).to_string()
}
