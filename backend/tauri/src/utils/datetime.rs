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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_get_current_time() {
        let current_time = get_current_time();
        assert!(current_time.len() == 19);
        assert!(current_time.contains('-'));
        assert!(current_time.contains(':'));
        assert!(current_time.contains(' '));
    }

    #[test]
    fn test_format_resource_system_time() {
        let test_time = Local.with_ymd_and_hms(2024, 3, 15, 14, 30, 45).unwrap();
        let system_time = test_time.into();
        let formatted_time = format_resource_system_time(system_time);
        assert_eq!(formatted_time, "2024/03/15");
    }
  
}