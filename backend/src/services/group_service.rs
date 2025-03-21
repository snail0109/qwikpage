use tauri::command;

use crate::{models::group::{GroupConfig, GroupList}, types::js_resp::JSResp};
use log;

// 查询所有分组信息
#[command]
pub fn load_groups() -> JSResp<GroupConfig> {
    log::debug!("load_groups");
    let config = GroupConfig::load();
    JSResp::from(config)
}

// 查询所有分组信息
#[command]
pub fn load_groups_with_projects(keyword: Option<String>) -> JSResp<GroupList> {
    log::debug!("load_groups_with_projects");
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
    log::debug!("add_group group_name: {}", group_name);
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
    log::debug!("edit_group id: {}, group_name: {}", id, group_name);
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
    log::debug!("delete_group id: {}", id);
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
