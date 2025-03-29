use std::fs;

use crate::utils::dirs::{ensure_dir_exists, get_app_data_path, get_config_path};
use anyhow::Result;

use super::dirs::{init_preference, projects_group_path};

// 初始化应用文件夹和文件
#[allow(unused_variables)]
pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用根目录
    get_config_path();

    init_preference();

    ensure_dir_exists(&get_app_data_path().join("code"));

    ensure_dir_exists(&get_app_data_path().join("build"));

    // 数据备份迁移
    // 如果 group.json 存在 修改文件名称为 projects.json
    let path = get_config_path().join("group.json");
    if path.exists() {
        let new_path = projects_group_path();
        fs::rename(path, new_path).unwrap();
    }

    // 读取 projects_group_path文件内容, 如果存在 groups 字段，并且 groups的值不是空数据
    // 如果 id 是 -1 则新增 is_default: true , 并且修改id为Uuid，如果id不是-1 新增is_default字段设置 false
    let projects_path = projects_group_path();
    log::debug!("修改默认项目分组数据，{:#?}", projects_path);
    if projects_path.exists() {
        let content = fs::read_to_string(&projects_path)?;
        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(groups) = json.get_mut("groups") {
                if let Some(groups_array) = groups.as_array() {
                    if !groups_array.is_empty() {
                        for group in groups.as_array_mut().unwrap() {
                            if let Some(id) = group.get("id") {
                                if id.as_str() == Some("-1") {
                                    log::debug!("修改默认项目分组id");
                                    group["is_default"] = serde_json::Value::Bool(true);
                                    group["id"] =
                                        serde_json::Value::String(uuid::Uuid::new_v4().to_string());
                                } else {
                                    if !group.get("is_default").is_some() {
                                        log::debug!("补充分组数据缺少的is_default字段");
                                        group["is_default"] = serde_json::Value::Bool(false);
                                    }
                                }
                            }
                        }
                        let new_content = serde_json::to_string_pretty(&json)?;
                        fs::write(&projects_path, new_content)?;
                    }
                }
            }
        }
    }

    Ok(())
}
