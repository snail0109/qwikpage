use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]
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
pub struct UpdateProject {
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
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub remark: String,
    pub count: usize,
    pub updated_at: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectList {
    pub list: Vec<ProjectSummary>,
    pub total: usize,
}

impl Project {
    pub fn new(id: String, name: String, remark: String, logo: String) -> Self {
        // 生成随机并且唯一的项目 ID
        let current_time = Local::now();
        Project {
            id,
            name,
            remark,
            logo,
            layout: 1,
            menu_mode: "horizontal".to_string(),
            menu_theme_color: "light".to_string(),
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

    pub fn update(&self, project_path: &Path, id: String, params: UpdateProject) {
        let mut project = Project::load(project_path);
        project.id = id;
        project.name = params.name;
        project.remark = params.remark;
        project.layout = params.layout;
        project.menu_mode = params.menu_mode;
        project.menu_theme_color = params.menu_theme_color;
        project.system_theme_color = params.system_theme_color;
        project.breadcrumb = params.breadcrumb;
        project.tag = params.tag;
        project.footer = params.footer;
        project.updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        project.save(project_path);
    }

    pub fn delete(project_path: &Path, mode: Option<String>) {
        // 删除项目目录 mode 如果等于 "all" 则删除项目目录
        if let Some(mode) = mode {
            if mode == "all" {
                fs::remove_dir_all(project_path).unwrap();
            }
        } else {
            let project_file = project_path.join("project.json");
            fs::remove_file(project_file).unwrap();
        }
    }
}
