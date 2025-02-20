use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{fs, io};

use crate::constans::PAGE_DIR;
use crate::utils::{get_app_root_dir, get_current_time};

use super::group::{GroupConfig, UpdateOption};

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
            MenuMode::Inline =>  "inline",
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
    pub name: String,                       // 项目名称
    pub remark: String,                     // 项目备注（可选）
    pub logo: String,                       // 项目 logo 的 URL（可选）
    pub layout: u32,                        // 系统布局 1 2
    pub menu_mode: String,                  // 菜单模式
    pub menu_theme_color: String,           // 菜单主题
    pub breadcrumb: bool,                   // 是否显示面包屑导航
    pub tag: bool,                          // 是否显示标签页
    pub footer: bool,                       // 是否显示页脚
    pub system_theme_color: Option<String>, // 系统主题颜色
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

// 项目配置文件
pub const PROJECT_CONFIG_FILE: &str = "project.json";

impl Project {
    pub fn new(id: String, name: String, remark: String, logo: String) -> Self {
        // 生成随机并且唯一的项目 ID
        Project {
            id,
            name,
            remark,
            logo,
            layout: ProjectLayout::LeftRight.to_value(),
            menu_mode: MenuMode::Vertical.to_str().to_string(),
            menu_theme_color: MenuThemeColor::Light.to_str().to_string(),
            breadcrumb: false,
            tag: false,
            footer: false,
            system_theme_color: None,
            created_at: get_current_time(),
            updated_at: get_current_time(),
        }
    }

    pub fn save(&self) -> Result<bool, Error> {
        let project_dir_path = get_app_root_dir().join(self.id.clone());
        if !project_dir_path.exists() {
            fs::create_dir_all(&project_dir_path)?;
        }
        let project_file = project_dir_path.join(PROJECT_CONFIG_FILE);
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(project_file, json)?;
        Ok(true)
    }

    pub fn load(project_id: String) -> io::Result<Self> {
        let project_file = get_app_root_dir().join(project_id).join(PROJECT_CONFIG_FILE);
        match fs::read_to_string(project_file) {
            Ok(data) => {
                let project: Project = serde_json::from_str(&data).unwrap();
                Ok(project)
            }
            Err(e) => Err(e),
        }
    }

    pub fn update(&mut self, params: ProjectUpdateParams) -> Result<bool, Error> {
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
        self.save();
        Ok(true)
    }

    pub fn delete(project_id: String, group_id: Option<String>) -> Result<bool, Error> {
        let project_dir = get_app_root_dir().join(&project_id);
        fs::remove_dir_all(project_dir)?;
        let mut config = GroupConfig::load()?;
        // 查找 config.groups 各个 group projects 是否包含 project_id
        let mut group_id = group_id;
        if let Some(group) = config.groups.iter_mut().find(|g| g.projects.as_ref().map_or(false, |projects| projects.contains(&project_id))) {
            group_id = Some(group.id.clone());
        }
        if let Some(group_id) = group_id {
            if let Err(e) = config.remove_project_from_group(group_id.clone(), project_id.clone()) {
                // 处理错误，例如记录日志或返回错误
                error!("Failed to remove project from group: {}", e);
                return Err(anyhow::anyhow!("Failed to remove project from group: {}", e));
            }
        }
        Ok(true)
        
    }

    pub fn count_pages_in_project(project_id: &str) -> usize {
        let page_dir = get_app_root_dir().join(&project_id).join(PAGE_DIR);
        // 目录不存在则返回 0
        if !page_dir.exists() {
            return 0;
        }
        let entries = fs::read_dir(page_dir).unwrap();
        let mut count = 0;
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if path.extension().unwrap() != "json" {
                    continue;
                }
                count += 1;
            }
        }
        count
    }

    pub fn add_project(group_id: Option<String>, name: String, remark: String, logo: String) -> Result<Project, Error> {
        let project_id = uuid::Uuid::new_v4().to_string();
        info!("add project: {}", &project_id);
        let project = Project::new(project_id.clone(), name, remark, logo);
        project.save()?;
        // group_id 不为空的时候，更新分组的项目列表
        if let Some(group_id) = group_id {
            let mut config = GroupConfig::load()?;
            config.update_group_project(group_id, project_id, UpdateOption::Remove);
        }
        Ok(project)
    }
    
}
