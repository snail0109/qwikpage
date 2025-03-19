use tauri::command;

use super::cmd_response::CmdResponse;
use crate::models::{
    config::Config,
    group::{GroupConfig, GroupList},
};
use log;

// 查询所有分组信息
#[command]
pub fn load_groups() -> CmdResponse<GroupConfig> {
    let config = GroupConfig::load();
    CmdResponse::from(config)
}

// 查询所有分组信息
#[command]
pub fn load_groups_with_projects(keyword: Option<String>) -> CmdResponse<GroupList> {
    let config = Config::global();
    log::debug!("project_path: {}", config.preferences().project_path);
    match GroupConfig::load() {
        Ok(config) => {
            let res = config.get_project_details(keyword);
            CmdResponse::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            CmdResponse::from(Err(e))
        }
    }
}

// 新增分组
#[command]
pub fn add_group(group_name: String) -> CmdResponse<String> {
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.add_group(group_name);
            CmdResponse::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            CmdResponse::from(Err(e))
        }
    }
}

// 修改分组
#[command]
pub fn edit_group(id: &str, group_name: String) -> CmdResponse<bool> {
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.update_group(id, Some(group_name));
            CmdResponse::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            CmdResponse::from(Err(e))
        }
    }
}

// 删除分组
#[command]
pub fn delete_group(id: &str) -> CmdResponse<bool> {
    match GroupConfig::load() {
        Ok(mut config) => {
            let res = config.delete_group(id);
            CmdResponse::from(res)
        }
        Err(e) => {
            // 处理加载配置失败的情况，返回一个错误响应
            CmdResponse::from(Err(e))
        }
    }
}
