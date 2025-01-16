use chrono::Local;

use super::constans::DATA_FORMAT;

pub fn get_current_time() -> String {
    Local::now().format(DATA_FORMAT).to_string()
}