use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::utils::constans::PROJECT_CONFIG_FILE;
use crate::utils::help::get_current_time;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,                         // 项目唯一标识
    pub name: String,                       // 项目名称
    pub remark: String,                     // 项目备注（可选）
    pub logo: String,                       // 项目 logo 的 URL（可选）
    pub layout: u32,                        // 系统布局 1 2
    pub menu_mode: String,                  // 菜单模式
    pub menu_theme_color: String,           // 菜单主题
    pub breadcrumb: bool,                   // 是否显示面包屑导航
    pub tag: bool,                          // 是否显示标签页
    pub footer: bool,                       // 是否显示页脚
    pub system_theme_color: Option<String>, // 系统主题
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectUpdateParams {
    pub name: String,                       // 项目名称
    pub remark: String,                     // 项目备注（可选）
    pub layout: u32,                        // 系统布局 1 2
    pub menu_mode: String,                  // 菜单模式
    pub menu_theme_color: String,           // 菜单主题
    pub breadcrumb: bool,                   // 是否显示面包屑导航
    pub tag: bool,                          // 是否显示标签页
    pub footer: bool,                       // 是否显示页脚
    pub system_theme_color: Option<String>, // 系统主题
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub remark: String,
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

impl Project {
    pub fn new(id: String, name: String, remark: String, logo: String) -> Self {
        // 生成随机并且唯一的项目 ID
        Project {
            id,
            name,
            remark,
            logo,
            layout: 1,
            menu_mode: "vertical".to_string(),
            menu_theme_color: "light".to_string(),
            breadcrumb: false,
            tag: false,
            footer: false,
            system_theme_color: None,
            created_at: get_current_time(),
            updated_at: get_current_time(),
        }
    }

    pub fn save(&self, project_path: &Path) {
        let project_file = project_path.join(PROJECT_CONFIG_FILE);
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(project_file, json).unwrap();
    }

    pub fn load(project_path: &Path) -> Self {
        let project_file = project_path.join(PROJECT_CONFIG_FILE);
        let json = fs::read_to_string(project_file).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn update(&mut self, params: ProjectUpdateParams) {
        self.name = params.name;
        self.remark = params.remark;
        self.layout = params.layout;
        self.menu_mode = params.menu_mode;
        self.menu_theme_color = params.menu_theme_color;
        self.system_theme_color = params.system_theme_color;
        self.breadcrumb = params.breadcrumb;
        self.tag = params.tag;
        self.footer = params.footer;
        self.updated_at = get_current_time();
    }

    pub fn delete(project_path: &Path, mode: Option<String>) {
        // 删除项目目录 mode 如果等于 "all" 则删除项目目录
        if let Some(mode) = mode {
            if mode == "all" {
                fs::remove_dir_all(project_path).unwrap();
            }
        } else {
            let project_file = project_path.join(PROJECT_CONFIG_FILE);
            fs::remove_file(project_file).unwrap();
        }
    }
}
