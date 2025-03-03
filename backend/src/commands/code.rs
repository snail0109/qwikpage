use crate::code_generator::main::{export_code, ExportCodeParams};
use log::error;
use log::info;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use tauri::command;
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
pub async fn export_project(app: AppHandle, params: ExportCodeParams) -> Result<(), String> {
    info!("export project start");
    if let Err(e) = export_code(app, params).await {
        // 出现错误时记录日志并返回错误
        error!("导出项目失败: {}", e);
        return Err(format!("导出项目失败: {}", e));
    }
    Ok(())
}
