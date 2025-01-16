use crate::models::page::count_pages_in_project;
use crate::models::project::{Project, ProjectList, ProjectSummary, ProjectUpdateParams};
use crate::utils::dirs;
use anyhow::Result;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

const PROJECT_CONFIG_FILE: &str = "project.json";

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

// 获取项目列表
#[command]
pub fn get_project_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
) -> Result<ProjectList, String> {
    info!(
        "Project::get_project_list start, page_num: {}, page_size: {}, keyword: {:?}",
        page_num, page_size, keyword
    );
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
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
                            if !project.name.contains(keyword) && !project.remark.contains(keyword)
                            {
                                continue;
                            }
                        }
                        let count = count_pages_in_project(&project.id);
                        project_list.push(ProjectSummary {
                            id: project.id,
                            name: project.name,
                            remark: project.remark,
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
    let start = (page_num - 1) * page_size;
    let end = start + page_size;
    let end = end.min(project_list.len());

    let total = project_list.len();
    let list = project_list[start..end].to_vec();
    Ok(ProjectList { total, list })
}

// 获取项目详情
#[command]
pub fn get_project_detail(id: String) -> Result<Project, String> {
    info!("Project::get_project_detail start, id: {}", id);
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&id);
    if let Some(project) = load_project(&project_path) {
        Ok(project)
    } else {
        error!("project does not found");
        Err(format!("{} does not found", project_path.display()))
    }
}

// 新建项目
#[command]
pub fn add_project(name: String, remark: String, logo: String) -> Result<(), String> {
    info!(
        "Project::add_project start name: {}, remark: {}, logo: {}",
        name, remark, logo
    );
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_id = uuid::Uuid::new_v4().to_string();
    let project_dir = root_dir.join(&project_id);
    if project_dir.exists() {
        error!("project_dir already exists");
        return Err(format!("{} already exists", project_dir.display()));
    }
    fs::create_dir_all(&project_dir).unwrap();
    let project = Project::new(project_id, name, remark, logo);
    project.save(&project_dir);
    Ok(())
}

// 更新项目
#[command]
pub fn update_project(id: String, params: ProjectUpdateParams) -> Result<(), String> {
    info!(
        "Project::update_project start, id: {}, params: {:?}",
        id, params
    );
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&id);
    if !project_dir.exists() {
        error!("project does not found");
        return Err(format!("{} does not found", project_dir.display()));
    }
    let mut project = Project::load(&project_dir);
    project.update(params);
    project.save(&project_dir);
    Ok(())
}

// 删除项目
#[command]
pub fn delete_project(id: String, mode: Option<String>) -> Result<(), String> {
    info!(
        "Project::delete_project start, id: {}, mode: {:?}",
        id, mode
    );
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&id);
    if !project_dir.exists() {
        error!("project does not found");
        return Err(format!("{} does not found", project_dir.display()));
    }
    Project::delete(&project_dir, mode);
    Ok(())
}
