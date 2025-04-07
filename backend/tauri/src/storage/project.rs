use anyhow::Error;
use log;
use std::io::ErrorKind;
use std::path::Path;
use std::fs;

use crate::manager::preference_manager::PreferencesManager;
use crate::manager::project_group_manager::ProjectGroupManager;
use crate::types::project::{MenuMode, MenuThemeColor, Project, ProjectAddParams, ProjectLayout, ProjectList, ProjectSummary, ProjectUpdateParams};
use crate::utils::datetime::get_current_time;
use crate::utils::dirs::get_default_build_path;
use crate::utils::file::is_valid_file;
use crate::utils::paginate;

use super::resource::ResourceConfig;

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
        let root_dir = &PreferencesManager::get_project_path();
        let project_dir_path = root_dir.join(self.id.clone());
        if !project_dir_path.exists() {
            fs::create_dir_all(&project_dir_path)?;
        }
        let project_file = project_dir_path.join(PROJECT_CONFIG_FILE);
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(project_file, json)?;
        Ok(true)
    }

    pub fn load(project_id: String) -> Result<Self, String> {
        let root_dir = &PreferencesManager::get_project_path();
        let project_file = root_dir.join(project_id).join(PROJECT_CONFIG_FILE);
        log::info!("加载项目配置文件: {:#?}", project_file);
        match fs::read_to_string(&project_file) {
            Ok(data) => {
                let project: Project = serde_json::from_str(&data).unwrap();
                Ok(project)
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        log::error!("项目配置文件不存在: {:?}", project_file);
                        Err(format!("项目配置文件不存在: {:?}", project_file))
                    }
                    _ => Err(e.to_string())
                }
            }
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
                log::error!("Failed to save project: {}", e);
                Err(anyhow::anyhow!("Failed to save project: {}", e))
            }
        }
    }

    pub async fn delete(
        project_id: String,
        group_id: String,
        logo_url: String,
    ) -> Result<bool, Error> {
        let root_dir = &PreferencesManager::get_project_path();
        let project_dir = root_dir.join(&project_id);
        log::info!("删除项目:{:#?}", project_dir);
        tokio::fs::remove_dir_all(project_dir).await?;
        if let Err(e) = ProjectGroupManager::remove_project_from_current_group(group_id.clone(), project_id.clone()) {
            // 处理错误，例如记录日志或返回错误
            log::error!("从分组中删除项目失败: {}", e);
            return Err(anyhow::anyhow!(
                "从分组中删除项目失败: {}",
                e
            ));
        }
        ResourceConfig::delete_project_logo(logo_url).await?;
        Ok(true)
    }

    pub fn count_pages_in_project(project_id: &str) -> usize {
        log::debug!("ProjectStotage::count_pages_in_project: 统计项目页面数量: {}", project_id);
        let root_dir = &PreferencesManager::get_project_path();
        let page_dir = root_dir.join(&project_id).join("pages");
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
                log::info!("页面配置文件: {}", path.to_str().unwrap());
                if path.extension().unwrap() != "json" {
                    continue;
                }
                count += 1;
            }
        }
        count
    }

    pub fn get_project_list_by_option(keyword: Option<String>) -> Result<Vec<ProjectSummary>, String> {
        log::debug!("ProjectStorage::get_project_list_by_option : 根据项目名称查询项目列表, 项目名称({:?})", keyword);
        let root_dir = &PreferencesManager::get_project_path();
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
                                if !project.name.contains(keyword) {
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
                    log::warn!("解析项目文件失败: {}", e);
                    None
                }
            },
            Err(e) => {
                log::warn!("解析项目文件失败: {}", e);
                None
            }
        }
    } else {
        None
    }
}

pub fn paginated_query_project_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
) -> Result<ProjectList, String> {
    log::info!(
        "分页查询项目列表, page_num: {}, page_size: {}, keyword: {:?}",
        page_num, page_size, keyword
    );
    let root_dir = &PreferencesManager::get_project_path();
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
                            if !project.name.contains(keyword) {
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
    log::info!("新增项目, 项目ID({})", &project_id);
    let root_dir = &PreferencesManager::get_project_path();
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

    ProjectGroupManager::add_project_to_current_group(group_id.clone(), project_id.clone())
        .map_err(|e| {
            log::error!(
                "将项目({})添加到分组({})失败:{}",
                project_id, group_id, e
            );
            anyhow::anyhow!("将项目({})添加到分组({})失败:{}", project_id, group_id, e)
        })?;
    Ok(project)
}
