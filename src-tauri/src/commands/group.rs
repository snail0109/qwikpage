use tauri::command;

use super::cmd_response::CmdResponse;
use crate::models::group::{ConfigFile, GroupList};

// 查询所有分组信息
#[command]
pub fn load_groups() -> CmdResponse<ConfigFile> {
    let config = ConfigFile::load();
    CmdResponse::from(config)
}

// 查询所有分组信息
#[command]
pub fn load_groups_with_projects(name: Option<String>) -> CmdResponse<GroupList> {
    let config = ConfigFile::load().unwrap();
    let res = config.get_project_details(name);
    CmdResponse::from(res)
}

// 新增分组
#[command]
pub fn add_group(group_name: String) -> CmdResponse<String> {
    let mut config = ConfigFile::load().unwrap();
    let res = config.add_group(group_name);
    CmdResponse::from(res)
}

// 修改分组
#[command]
pub fn edit_group(id: &str, group_name: String) -> CmdResponse<bool> {
    let mut config = ConfigFile::load().unwrap();
    let res = config.update_group(id, Some(group_name), None);
    CmdResponse::from(res)
}

// 删除分组
#[command]
pub fn delete_group(id: &str) -> CmdResponse<bool> {
    let mut config = ConfigFile::load().unwrap();
    let res = config.delete_group(id);
    CmdResponse::from(res)
}
