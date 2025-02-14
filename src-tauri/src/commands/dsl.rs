
use tauri::command;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use crate::page::get_page_list;
use crate::core::code::export_page;
use crate::utils::get_app_root_dir;

#[command]
pub fn export_json(file_path: String, json_data: Value) -> Result<(), String> {
    // 将 JSON 数据转换为字符串
    let json_string = serde_json::to_string_pretty(&json_data)
        .map_err(|e| format!("JSON 序列化失败: {}", e))?;

    // 打开文件并写入数据
    let mut file = File::create(&file_path)
        .map_err(|e| format!("无法创建文件: {}", e))?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}

#[command]
pub fn export_project(id: String) {
    // 创建项目文件
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code").join(&id);
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir).expect("failed to create pages dir");
    }
    // TODO 生成 Fishx 项目模版


    // get_page_list 获取项目下的页面列表
    let page_list = get_page_list(1, 200, None, Some(id))
        .map_err(|e| format!("获取项目页面列表失败: {}", e)).unwrap(); 

    // 遍历页面列表，导出页面数据, 获取页面数据序列
    for (index, page) in page_list.list.iter().enumerate() {
        export_page(index, code_dir.clone(), page.clone());
    }

}