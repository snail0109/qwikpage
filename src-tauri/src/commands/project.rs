use crate::commands::cmd_response::CmdResponse;
use crate::models::project::{
    Project, ProjectList, ProjectSummary, ProjectUpdateParams, PROJECT_CONFIG_FILE,
};
use crate::utils::{get_app_root_dir, paginate};
use anyhow::Result;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

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
    let root_dir: PathBuf = get_app_root_dir();
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
                        let count = Project::count_pages_in_project(&project.id);
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
    let (list, total) = paginate(project_list, page_num, page_size);
    Ok(ProjectList { total, list })
}

// 获取项目详情
#[command]
pub fn get_project_detail(id: String) -> CmdResponse<Project> {
    info!("Project::get_project_detail start, id: {}", id);
    CmdResponse::from(Project::load(id))
}

// 新建项目
#[command]
pub fn add_project(
    group_id: Option<String>,
    name: String,
    remark: String,
    logo: String,
) -> CmdResponse<Project> {
    info!(
        "Project::add_project start, name: {}, remark: {}, logo: {}",
        name, remark, logo
    );
    let project = Project::add_project(group_id, name, remark, logo);
    CmdResponse::from(project)
}

// 更新项目
#[command]
pub fn update_project(id: String, params: ProjectUpdateParams) -> CmdResponse<bool> {
    info!(
        "Project::update_project start, id: {}, params: {:?}",
        id, params
    );
    let mut project = Project::load(id).unwrap();
    let res = project.update(params);
    CmdResponse::from(res)
}

// 删除项目
#[command]
pub fn delete_project(id: String, group_id: Option<String>) -> CmdResponse<bool>{
    info!("Project::delete_project start, id: {}", id);
    let res = Project::delete(id, group_id);
    CmdResponse::from(res)
}

// 获取项目列表
#[command]
pub fn get_project_list_new(keyword: Option<String>) -> Result<Vec<ProjectSummary>, String> {
    info!("Project::get_project_list start, keyword: {:?}", keyword);
    let root_dir: PathBuf = get_app_root_dir();
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
                        let count = Project::count_pages_in_project(&project.id);
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
    Ok(project_list)
}
