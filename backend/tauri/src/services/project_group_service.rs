use tauri::command;

use crate::manager::project_group_manager::ProjectGroupManager;
use crate::types::group::ProjectGroup;
use crate::types::{group::GroupWithProjectList, js_resp::JSResp};
use log;


// 查询所有分组信息
#[command]
pub fn load_groups_with_projects(keyword: Option<String>) -> JSResp<GroupWithProjectList> {
    log::debug!(
        "TProjectGroupService::load_groups_with_projects(): 查询分组项目信息，项目名称({:?})",
        Some(&keyword)
    );
    let res = ProjectGroupManager::get_current_project_details(keyword);
    JSResp::from(res)
}

// 新增分组
#[command]
pub fn add_group(group_name: String) -> JSResp<ProjectGroup> {
    log::debug!("TProjectGroupService::add_group(): 分组名称({})", group_name);
    let res = ProjectGroupManager::add_group_to_current_project(group_name, false);
    match &res {
        Ok(group) => {
            log::info!("新增分组成功: {:?}", group);
        }
        Err(e) => {
            log::error!("新增分组失败: {:?}", e);
        }
    }
    JSResp::from(res)
}

// 修改分组
#[command]
pub fn edit_group(id: &str, group_name: String) -> JSResp<Option<ProjectGroup>> {
    log::debug!(
        "TProjectGroupService::edit_group(): 修改分组 id({}), 分组名:({})",
        id,
        group_name
    );
    let res = ProjectGroupManager::update_current_project_group(id, group_name);
    match &res {
        Ok(_) => {
            log::info!("修改分组成功");
        }
        Err(e) => {
            log::error!("修改分组失败: {:?}", e);
        }
    }
    JSResp::from(res)
}

// 删除分组
#[command]
pub fn delete_group(id: &str) -> JSResp<Option<ProjectGroup>> {
    log::debug!("TProjectGroupService::delete_group(): 删除分组:({})", id);
    let res = ProjectGroupManager::remove_group_from_current_project(id);
    match &res {
        Ok(_) => {
            log::info!("删除分组成功");
        }
        Err(e) => {
            log::error!("删除分组失败: {:?}", e);
        }
    }
    JSResp::from(res)
}

