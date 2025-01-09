use crate::models::project::{Project, ProjectSummary};
use crate::utils::dirs;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use log::{info, error};
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
pub fn get_project_list() -> Vec<ProjectSummary> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let mut project_list = Vec::new();

    if let Ok(entries) = fs::read_dir(&root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let project_path = entry.path();
                if project_path.is_dir() {
                    if let Some(project) = load_project(&project_path) {
                        let page_count = count_pages_in_project(&project_path);
                        project_list.push(ProjectSummary {
                            name: project.name,
                            remark: project.remark,
                            page_count,
                        });
                    }
                }
            }
        }
    }

    project_list
}

// 获取项目详情
#[command]
pub fn get_project_detail(project_id: String) -> Result<Project, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&project_id);
    if let Some(project) = load_project(&project_path) {
        Ok(project)
    } else {
        Err(format!("{} does not found", project_path.display()))
    }
}

// 新建项目
#[command]
pub fn add_project(project_name: String, project_remark: String) -> Result<(), String> {
    info!("add_project");
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&project_name);
    if project_dir.exists() {
        error!("project_dir already exists");
        return Err(format!("{} already exists", project_dir.display()))
    }
    fs::create_dir_all(&project_dir).unwrap();
    let project = Project::new(project_name, project_remark);
    project.save(&project_dir);
    println!("Project created successfully");
    Ok(())
}


// 更新项目
#[command]
pub fn update_project(project_id: String, project_name: String, project_remark: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_dir = root_dir.join(&project_id);
    if !project_dir.exists() {
        return Err(format!("{} does not found", project_dir.display()))
    }
    let mut project = Project::load(&project_dir);
    project.name = project_name;
    project.remark = project_remark;
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
        return Err(format!("{} does not found", project_dir.display()))
    }
    Project::delete(&project_dir);
    println!("Project deleted successfully");
    Ok(())
}