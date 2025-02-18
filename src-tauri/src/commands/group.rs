use tauri::command;

use crate::models::group::ConfigFile;
use super::cmd_response::CmdResponse;

// 查询
// #[command]
// pub fn load_groups(group_name: String) -> CmdResponse<String> {
//     let mut config = ConfigFile::load().unwrap();
//     let res = config.add_group(
//         group_name,
//     );
//     CmdResponse::from(res)
// }


// 新增分组
#[command]
pub fn add_group(group_name: String) -> CmdResponse<String> {
    let mut config = ConfigFile::load().unwrap();
    let res = config.add_group(
        group_name,
    );
    CmdResponse::from(res)
}

// 修改分组
#[command]
pub fn edit_group(id: &str, name: String) -> CmdResponse<bool>  {
    let mut config = ConfigFile::load().unwrap();
    let res = config.update_group(
        id,
        Some(name),
        None
    );
    CmdResponse::from(res)
}

// 删除分组
#[command]
pub fn delete_group(id: &str) -> CmdResponse<bool> {
    let mut config = ConfigFile::load().unwrap();
    let res = config.delete_group(
        id,
    );
    CmdResponse::from(res)
}
