use tauri::command;

use crate::types::{group::{GroupConfig, GroupList}, js_resp::JSResp};
use log;

// 查询所有分组信息
#[command]
pub fn load_groups() -> JSResp<GroupConfig> {
    log::debug!("TGroupService::load_groups: 查询项目分组");
    let config = GroupConfig::load();
    JSResp::from(config)
}

// 查询所有分组信息
#[command]
pub fn load_groups_with_projects(keyword: Option<String>) -> JSResp<GroupList> {
    log::debug!("TGroupService::load_groups_with_projects(): 查询分组项目信息，项目名称({:?})", Some(&keyword));
    match GroupConfig::load() {
        Ok(config) => {
            let res = config.get_project_details(keyword);
            JSResp::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            JSResp::from(Err(e))
        }
    }
}

// 新增分组
#[command]
pub fn add_group(group_name: String) -> JSResp<String> {
    log::debug!("TGroupService::add_group(): 分组名称({})", group_name);
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.add_group(group_name);
            JSResp::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            JSResp::from(Err(e))
        }
    }
}

// 修改分组
#[command]
pub fn edit_group(id: &str, group_name: String) -> JSResp<bool> {
    log::debug!("TGroupService::edit_group(): 分组 id({}), 分组名:({})", id, group_name);
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.update_group(id, Some(group_name));
            JSResp::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            JSResp::from(Err(e))
        }
    }
}

// 删除分组
#[command]
pub fn delete_group(id: &str) -> JSResp<bool> {
    log::debug!("TGroupService::delete_group(): 删除分组:({})", id);
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.delete_group(id);
            JSResp::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            JSResp::from(Err(e))
        }
    }
}
