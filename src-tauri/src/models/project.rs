use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Layout {
    Layout1 = 1, // 菜单和内容左右布局
    Layout2 = 2, // 菜单和内容上下布局
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MenuMode {
    Horizontal, // 水平菜单
    Vertical,   // 垂直菜单
    Inline,     // 内嵌菜单
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MenuThemeColor {
    Dark,  // 深色主题
    Light, // 浅色主题
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,                       // 项目唯一标识
    pub name: String,                     // 项目名称
    pub remark: String,                   // 项目备注（可选）
    pub logo: Option<String>,             // 项目 logo 的 URL（可选）
    pub layout: Layout,                   // 系统布局
    pub menu_mode: MenuMode,              // 菜单模式
    pub menu_theme_color: MenuThemeColor, // 菜单主题
    pub breadcrumb: bool,                 // 是否显示面包屑导航
    pub tag: bool,                        // 是否显示标签页
    pub footer: bool,                     // 是否显示页脚
    pub system_theme_color: Option<String>,       // 系统主题
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectSummary {
    pub name: String,
    pub remark: String,
    pub page_count: usize,
}


impl Project {
    pub fn new(name: String, remark: String) -> Self {
        // 生成随机并且唯一的项目 ID
        let current_time = Utc::now().naive_utc();
        Project {
            id: Uuid::new_v4().to_string(),
            name,
            remark,
            logo: None,
            layout: Layout::Layout1,
            menu_mode: MenuMode::Horizontal,
            menu_theme_color: MenuThemeColor::Dark,
            breadcrumb: true,
            tag: true,
            footer: true,
            system_theme_color: None,
            created_at: current_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: current_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn save(&self, project_path: &Path) {
        let project_file = project_path.join("project.json");
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(project_file, json).unwrap();
    }

    pub fn load(project_path: &Path) -> Self {
        let project_file = project_path.join("project.json");
        let json = fs::read_to_string(project_file).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn update(&self, project_path: &Path) {
        self.save(project_path);
    }

    pub fn delete(project_path: &Path) {
        // 删除项目目录
        fs::remove_dir_all(project_path).unwrap();
    }
}
