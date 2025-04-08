use serde::{Deserialize, Serialize};

use crate::utils::dirs::get_default_code_path;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub theme: String,        // 主题
    pub language: String,     // 语言
    pub font_size: u32,       // 字体大小
    pub font_bold: String,    // 是否粗体
    pub font_family: String,  // 字体
    pub check_update: bool,   // 自动更新
    pub project_path: String, // DSL代码目录
}


const DEFAULT_FONT_SIZE: u32 = 12;
const DEFAULT_FONT_BOLD: &str = "normal";

impl Default for Preferences {
    fn default() -> Self {
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei Mono".to_string()
        };
        Self {
            font_family,
            theme: "auto".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            check_update: true,
            project_path: get_default_code_path(),
        }
    }
}
