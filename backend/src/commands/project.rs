use crate::commands::cmd_response::CmdResponse;
use crate::models::project::{
    get_project_list_inner, Project, ProjectAddParams, ProjectList, add_project_inner, ProjectUpdateParams
};
use crate::models::resource::{ResourceConfig, UploadResourceParams};
use anyhow::Result;
use log::info;
use std::path::PathBuf;
use tauri::command;


// 获取项目列表
#[command]
pub fn get_project_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
) -> Result<ProjectList, String> {
    get_project_list_inner(page_num, page_size, keyword)
}

// 获取项目详情
#[command]
pub fn get_project_detail(id: String) -> CmdResponse<Project> {
    info!("Project::get_project_detail start, id: {}", id);
    CmdResponse::from(Project::load(id))
}

// 新建项目
#[command]
pub fn add_project(params: ProjectAddParams) -> CmdResponse<Project> {
    info!("Project::add_project start, params: {:#?}", params);
    let project = add_project_inner(params);
    CmdResponse::from(project)
}

// 更新项目
#[command]
pub fn update_project(params: ProjectUpdateParams) -> CmdResponse<bool> {
    info!("Project::update_project start, params: {:#?}", params);
    let mut project = Project::load(params.id.clone()).unwrap();
    let res = project.update(params);
    CmdResponse::from(res)
}

// 删除项目
#[command]
pub async fn delete_project(id: String, group_id: String, logo_url: String) -> CmdResponse<bool> {
    info!("Project::delete_project start, id: {}", id.clone());
    let res = Project::delete(id, group_id, logo_url).await;
    CmdResponse::from(res)
}

// 修改项目logo
#[command]
pub async fn upload_project_resource(params: UploadResourceParams) -> Result<PathBuf, String> {
    info!("Project::upload_project_resource, params: {:#?}", params);
    let res = ResourceConfig::upload_project_resource(params);
    res.await.map_err(| op | op.to_string())
}