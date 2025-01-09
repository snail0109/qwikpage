use crate::models::project::{Project, ProjectList, ProjectSummary};
use crate::utils::dirs;
use anyhow::Result;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

// 加载项目详情信息
fn load_project(project_path: &Path) -> Option<Project> {
    let project_file = project_path.join("project.json");
    if project_file.exists() {
        let json = fs::read_to_string(project_file).unwrap();
        let project: Project = serde_json::from_str(&json).unwrap();
        Some(project)
    } else {
        None
    }
}

// 统计项目中的页面数量
fn count_pages_in_project(project_path: &Path) -> usize {
    let pages_dir = project_path.join("pages");
    if pages_dir.exists() {
        fs::read_dir(pages_dir).unwrap().count()
    } else {
        0
    }
}

// 获取项目列表
#[command]
pub fn get_project_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
) -> Result<ProjectList, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let mut project_list = Vec::new();

    if let Ok(entries) = fs::read_dir(&root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let project_path = entry.path();
                if project_path.is_dir() {
                    if let Some(project) = load_project(&project_path) {
                        // 如果keyword传入了值，只返回匹配的项目
                        if let Some(keyword) = &keyword {
                            if !project.name.contains(keyword) && !project.remark.contains(keyword) {
                                continue;
                            }
                        }

                        let count = count_pages_in_project(&project_path);
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
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&id);
    if let Some(project) = load_project(&project_path) {
        Ok(project)
    } else {
        Err(format!("{} does not found", project_path.display()))
    }
}

// 新建项目
#[command]
pub fn add_project(name: String, remark: String, logo: String) -> Result<(), String> {
    info!("add_project");
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
    println!("Project created successfully");
    Ok(())
}

// 更新项目
#[command]
pub fn update_project(project_id: String, name: String, remark: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&project_id);
    if !project_dir.exists() {
        return Err(format!("{} does not found", project_dir.display()));
    }
    let mut project = Project::load(&project_dir);
    project.name = name;
    project.remark = remark;
    project.save(&project_dir);
    println!("Project updated successfully");
    Ok(())
}

// 删除项目
#[command]
pub fn delete_project(project_id: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&project_id);
    if !project_dir.exists() {
        return Err(format!("{} does not found", project_dir.display()));
    }
    Project::delete(&project_dir);
    println!("Project deleted successfully");
    Ok(())
}
