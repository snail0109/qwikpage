use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf};
use zip;
use tokio::fs as async_fs;

const REPLACEMENT_CHARACTER: &str = "##replace##";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExportType {
    Fishx,
    Fish,
    Vue,
}


// 将 JSON 值转换为 JavaScript 表示的字符串
pub fn value_to_js(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", escape_string(s)),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_js).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, value_to_js(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

// 转义字符串中的特殊字符
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}



pub async fn download_temp(code_dir: &PathBuf, export_type: &ExportType) -> Result<(), String> {
    // 根据 export_type 设置 template_url
    info!("export_type: {:#?}", export_type);
    let template_url = match export_type {
        ExportType::Fishx => String::from("https://fish.iwhalecloud.com/qwikpage-fishx/app.zip"),
        ExportType::Vue => String::from("https://fish.iwhalecloud.com/qwikpage-vue3/app.zip"),
        ExportType::Fish => String::from("https://fish.iwhalecloud.com/qwikpage-fish/app.zip"),
    };
    // 下载代码模板
    let template_path = code_dir.join("fishx-template.zip");

    info!("下载代码模板......");
    let response =
        reqwest::get(template_url)
        .await
        .map_err(|e| format!("下载模板失败: {}", e))?;
    let content = response
        .bytes()
        .await
        .map_err(|e| format!("读取响应内容失败: {}", e))?;

    info!("保存zip文件");
    async_fs::write(&template_path, content)
    .await
    .map_err(|e| format!("保存模板文件失败: {}", e))?;

    info!("解压文件");
    let file = fs::File::open(&template_path).map_err(|e| format!("打开zip文件失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("读取zip文件失败: {}", e))?;
    extract_archive(&mut archive, code_dir)?;

    info!("删除zip文件");
    fs::remove_file(&template_path).map_err(|e| format!("删除zip文件失败: {}", e))?;

    // mac 下会生成 __MACOSX 文件
    #[cfg(target_os = "macos")]
    remove_macosx_folder(&template_path)?;

    Ok(())
}

fn extract_archive(archive: &mut zip::ZipArchive<fs::File>, code_dir: &PathBuf) -> Result<(), String> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("访问zip文件条目失败: {}", e))?;
        let outpath = code_dir.join(file.name());
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("创建目录失败: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| format!("创建父目录失败: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| format!("复制文件内容失败: {}", e))?;
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn remove_macosx_folder(template_path: &PathBuf) -> Result<(), String> {
    let macosx_path = template_path.with_file_name("__MACOSX");
    if macosx_path.exists() {
        fs::remove_dir_all(macosx_path).map_err(|e| format!("删除macosx文件夹失败: {}", e))?;
    }
    Ok(())
}