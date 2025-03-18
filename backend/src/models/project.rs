use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};

use crate::utils::datetime::get_current_time;
use crate::utils::dirs::{get_default_build_path, PAGE_DIR};
use crate::utils::file::is_valid_file;
use crate::utils::paginate;

use super::config::Config;
use super::group::GroupConfig;
use super::resource::ResourceConfig;

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

// 项目配置文件
pub const PROJECT_CONFIG_FILE: &str = "project.json";


impl Project {
    pub fn new(
        id: String,
        name: String,
        theme_color: String,
        remark: Option<String>,
        logo: String,
        group_id: String,
    ) -> Self {
        // 生成随机并且唯一的项目 ID
        Project {
            id,
            group_id,
            name,
            remark,
            logo,
            theme_color,
            layout: ProjectLayout::LeftRight.to_value(),
            menu_mode: MenuMode::Vertical.to_str().to_string(),
            menu_theme_color: MenuThemeColor::Light.to_str().to_string(),
            breadcrumb: false,
            tag: false,
            footer: false,
            system_theme_color: None,
            created_at: get_current_time(),
            updated_at: get_current_time(),
            code_export_path: get_default_build_path(),
        }
    }

    pub fn save(&self) -> Result<bool, Error> {
        let root_dir = &Config::global().preferences().get_project_path();
        let project_dir_path = root_dir.join(self.id.clone());
        if !project_dir_path.exists() {
            fs::create_dir_all(&project_dir_path)?;
        }
        let project_file = project_dir_path.join(PROJECT_CONFIG_FILE);
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(project_file, json)?;
        Ok(true)
    }

    pub fn load(project_id: String) -> io::Result<Self> {
        let root_dir = &Config::global().preferences().get_project_path();
        let project_file = root_dir
            .join(project_id)
            .join(PROJECT_CONFIG_FILE);
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
        self.theme_color = params.theme_color;
        self.menu_mode = params.menu_mode;
        self.menu_theme_color = params.menu_theme_color;
        self.system_theme_color = params.system_theme_color;
        self.code_export_path = params.code_export_path;
        self.breadcrumb = params.breadcrumb;
        self.tag = params.tag;
        self.footer = params.footer;
        if let Some(logo) = params.logo {
            self.logo = logo;
        }
        self.updated_at = get_current_time();
        match self.save() {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Failed to save project: {}", e);
                Err(anyhow::anyhow!("Failed to save project: {}", e))
            }
        }
    }

    pub async fn delete(project_id: String, group_id: String, logo_url: String) -> Result<bool, Error> {
        let root_dir = &Config::global().preferences().get_project_path();
        let project_dir = root_dir.join(&project_id);
        tokio::fs::remove_dir_all(project_dir).await?;
        let mut config = GroupConfig::load()?;
        if let Err(e) = config.remove_project_from_group(group_id.clone(), project_id.clone()) {
            // 处理错误，例如记录日志或返回错误
            error!("Failed to remove project from group: {}", e);
            return Err(anyhow::anyhow!(
                "Failed to remove project from group: {}",
                e
            ));
        }
        ResourceConfig::delete_project_logo(logo_url).await?;
        Ok(true)
    }

    pub fn count_pages_in_project(project_id: &str) -> usize {
        let root_dir = &Config::global().preferences().get_project_path();
        let page_dir = root_dir.join(&project_id).join(PAGE_DIR);
        // 目录不存在则返回 0
        if !page_dir.exists() {
            return 0;
        }
        let entries = fs::read_dir(page_dir).unwrap();
        let mut count = 0;
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if is_valid_file(&path) {
                info!("entry path: {}", path.to_str().unwrap());
                if path.extension().unwrap() != "json" {
                    continue;
                }
                count += 1;
            }
        }
        count
    }


    pub fn get_project_list_inner(keyword: Option<String>) -> Result<Vec<ProjectSummary>, String> {
        info!("Project::get_project_list start, keyword: {:?}", keyword);
        let root_dir = &Config::global().preferences().get_project_path();
        let mut project_list = Vec::new();
    
        if let Ok(entries) = fs::read_dir(&root_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let project_path = entry.path();
                    // 过滤页面目录
                    if project_path
                        .file_name()
                        .map_or(false, |name| name == "page")
                    {
                        continue;
                    }
                    if project_path.is_dir() {
                        if let Some(project) = load_project(&project_path) {
                            // 如果keyword传入了值，只返回匹配的项目
                            if let Some(keyword) = &keyword {
                                if !project.name.contains(keyword)
                                {
                                    continue;
                                }
                            }
                            let count = Project::count_pages_in_project(&project.id);
                            project_list.push(ProjectSummary {
                                id: project.id,
                                name: project.name,
                                remark: project.remark,
                                theme_color: project.theme_color,
                                updated_at: project.updated_at,
                                logo: project.logo,
                                count,
                            });
                        }
                    }
                }
            }
        }
        Ok(project_list)
    }
    
}


// 加载项目详情信息
fn load_project(project_path: &Path) -> Option<Project> {
    let project_file = project_path.join(PROJECT_CONFIG_FILE);
    if project_file.exists() {
        match fs::read_to_string(&project_file) {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(project) => Some(project),
                Err(e) => {
                    error!("Failed to deserialize project file: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to read project file: {}", e);
                None
            }
        }
    } else {
        None
    }
}


pub fn get_project_list_inner(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
) -> Result<ProjectList, String> {
    info!(
        "Project::get_project_list start, page_num: {}, page_size: {}, keyword: {:?}",
        page_num, page_size, keyword
    );
    let root_dir = &Config::global().preferences().get_project_path();
    let mut project_list = Vec::new();

    if let Ok(entries) = fs::read_dir(&root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let project_path = entry.path();
                // 过滤页面目录
                if project_path
                    .file_name()
                    .map_or(false, |name| name == "page")
                {
                    continue;
                }
                if project_path.is_dir() {
                    if let Some(project) = load_project(&project_path) {
                        // 如果keyword传入了值，只返回匹配的项目
                        if let Some(keyword) = &keyword {
                            if !project.name.contains(keyword)
                            {
                                continue;
                            }
                        }
                        let count = Project::count_pages_in_project(&project.id);
                        project_list.push(ProjectSummary {
                            id: project.id,
                            name: project.name,
                            remark: project.remark,
                            theme_color: project.theme_color,
                            updated_at: project.updated_at,
                            logo: project.logo,
                            count,
                        });
                    }
                }
            }
        }
    }
    // 分页逻辑
    let (list, total) = paginate(project_list, page_num, page_size);
    Ok(ProjectList { total, list })
}

pub fn add_project_inner(params: ProjectAddParams) -> Result<Project, Error> {
    let project_id = uuid::Uuid::new_v4().to_string();
    let group_id = params.group_id.clone();
    info!("add project: {}", &project_id);
    let root_dir = &Config::global().preferences().get_project_path();
    let project_dir_path = root_dir.join(project_id.clone());
    if !project_dir_path.exists() {
        fs::create_dir_all(&project_dir_path)?;
    }
    let project = Project::new(
        project_id.clone(),
        params.name,
        params.theme_color,
        params.remark,
        params.logo,
        group_id.clone(),
    );
    project.save()?;

    // group_id 为 None 时，添加到默认分组
    let mut config = GroupConfig::load().map_err(|e| {
        error!("Failed to load group configuration: {}", e);
        anyhow::anyhow!("加载分组配置失败: {}", e)
    })?;
    config
        .add_group_project(group_id.clone(), project_id.clone())
        .map_err(|e| {
            error!(
                "Failed to add project {} to group {}: {}",
                project_id, group_id, e
            );
            anyhow::anyhow!("添加项目 {} 到分组 {} 失败: {}", project_id, group_id, e)
        })?;
    Ok(project)
}
