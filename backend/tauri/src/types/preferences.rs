use serde::{Deserialize, Serialize};

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