use serde::{Deserialize, Serialize};

// 系统布局
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProjectLayout {
    LeftRight,
    TopBottom,
}

impl ProjectLayout {
    pub fn to_value(&self) -> u32 {
        match self {
            ProjectLayout::LeftRight => 1,
            ProjectLayout::TopBottom => 2,
        }
    }
}

// 菜单模式
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MenuMode {
    // 垂直水平内嵌
    Vertical,
    Horizontal,
    Inline,
}

impl MenuMode {
    pub fn to_str(&self) -> &'static str {
        match self {
            MenuMode::Vertical => "vertical",
            MenuMode::Horizontal => "horizontal",
            MenuMode::Inline => "inline",
        }
    }
}

// 菜单主题
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MenuThemeColor {
    Dark,
    Light,
}

impl MenuThemeColor {
    pub fn to_str(&self) -> &'static str {
        match self {
            MenuThemeColor::Dark => "dark",
            MenuThemeColor::Light => "light",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,                         // 项目唯一标识
    pub group_id: String,                   // 项目分组唯一标识
    pub name: String,                       // 项目名称
    pub remark: Option<String>,             // 项目备注（可选）
    pub logo: String,                       // 项目 logo 的 URL（可选）
    pub theme_color: String,                // 项目主题色
    pub layout: u32,                        // 系统布局 1 2
    pub menu_mode: String,                  // 菜单模式
    pub menu_theme_color: String,           // 菜单主题
    pub breadcrumb: bool,                   // 是否显示面包屑导航
    pub tag: bool,                          // 是否显示标签页
    pub footer: bool,                       // 是否显示页脚
    pub system_theme_color: Option<String>, // 系统主题颜色
    pub variables: Option<String>,          // 项目变量
    pub created_at: String,
    pub updated_at: String,
    pub code_export_path: String,   // 代码导出路径
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectUpdateParams {
    pub id: String,
    pub name: String,                       // 项目名称
    pub remark: Option<String>,                     // 项目备注（可选）
    pub layout: u32,                        // 系统布局 1 2
    pub theme_color: String,                // 项目主题
    pub menu_mode: String,                  // 菜单模式
    pub menu_theme_color: String,           // 菜单主题
    pub breadcrumb: bool,                   // 是否显示面包屑导航
    pub tag: bool,                          // 是否显示标签页
    pub footer: bool,                       // 是否显示页脚
    pub system_theme_color: Option<String>, // 系统主题
    pub logo: Option<String>, // 系统主题
    pub code_export_path: String,   // 代码导出路径
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub remark: Option<String>,
    pub theme_color: String,
    pub system_theme_color: Option<String>,
    pub count: usize,
    pub updated_at: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectList {
    pub list: Vec<ProjectSummary>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectAddParams {
    pub group_id: String,
    pub name: String,
    pub remark: Option<String>,
    pub logo: String,
    pub theme_color: String,
}
