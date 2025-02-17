use crate::core::code::{download_temp, export_page, handle_routes};
use crate::models::page::Page;
use crate::utils::get_app_root_dir;
use log::info;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use tauri::command;
use tauri_plugin_opener::OpenerExt;
use tauri::AppHandle;

#[command]
pub fn export_json(file_path: String, json_data: Value) -> Result<(), String> {
    // 将 JSON 数据转换为字符串
    let json_string =
        serde_json::to_string_pretty(&json_data).map_err(|e| format!("JSON 序列化失败: {}", e))?;

    // 打开文件并写入数据
    let mut file = File::create(&file_path).map_err(|e| format!("无法创建文件: {}", e))?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}

#[command]
pub fn export_project(app: AppHandle, id: String) -> Result<(), String> {
    // 创建项目文件
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code").join(&id);

    info!("code_dir: {:?}", code_dir);
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // TODO  处理异常
    download_temp(&code_dir).unwrap();

    info!("查询项目页面信息: {:?}", id);
    let page_list = Page::list_with_options(Some(id), None).unwrap();

    let mut index = 1;
    let page_len  = page_list.len();
    if page_len == 0 {
        info!("项目没有页面");
        return Ok(());
    }
    info!("页面数量: {:?}", page_len);

    // 遍历页面列表并导出每个页面
    for page in &page_list {
        export_page(index, code_dir.clone(), page);
        index += 1;
    }

    // 这边处理路由文件
    handle_routes(code_dir.clone(), page_len, &page_list);

    app.opener()
        .open_path(code_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())?;

    Ok(())
}
