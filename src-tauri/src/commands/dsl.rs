use tauri::command;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{Write, Read};
use crate::page::get_page_list;
use crate::core::code::export_page;
use crate::utils::get_app_root_dir;
use log::{error, info, warn};
use reqwest;
use zip;
use crate::models::page::{Page, PageList};

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
pub fn export_project(id: String) -> Result<(), String> {
    // 创建项目文件
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code").join(&id);
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    info!("code_dir: {}", code_dir.display());
    // 下载代码模板
    let template_url = "https://fish.iwhalecloud.com/qwikpage-fishx/app.zip";
    let template_path = code_dir.join("fishx-template.zip");

    // 使用 reqwest 下载文件
    let response = reqwest::blocking::get(template_url)
        .map_err(|e| format!("下载模板失败: {}", e))?;
    let content = response.bytes()
        .map_err(|e| format!("读取响应内容失败: {}", e))?;

    // 保存zip文件
    fs::write(&template_path, content)
        .map_err(|e| format!("保存模板文件失败: {}", e))?;

    // 解压文件
    let file = fs::File::open(&template_path)
        .map_err(|e| format!("打开zip文件失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("读取zip文件失败: {}", e))?;

    // 解压所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("访问zip文件条目失败: {}", e))?;
        let outpath = code_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("创建父目录失败: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("复制文件内容失败: {}", e))?;
        }
    }

    // 删除zip文件
    fs::remove_file(template_path)
        .map_err(|e| format!("删除zip文件失败: {}", e))?;

    Ok(())
}