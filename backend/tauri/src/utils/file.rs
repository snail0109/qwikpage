use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, Result, Error};
use serde::{de::DeserializeOwned, Serialize};

use crate::types::resource::FontMeta;

// 判断 path 是有效文件，忽略隐藏文件
pub fn is_valid_file(path: &PathBuf) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.starts_with(".") {
        return false;
    }
    path.is_file()
}


// 格式化文件大小
pub fn format_file_size(size: u64) -> String {
    #[cfg(target_os = "macos")]
    let use_base_10 = true; // macOS 使用 base-10

    #[cfg(not(target_os = "macos"))]
    let use_base_10 = false; // 其他系统 (如 Windows) 使用 base-2

    if use_base_10 {
        // Base-10 (decimal)
        if size < 1000 {
            format!("{} B", (size as f64 / 1000.0).round() as u64)
        } else if size < 1000 * 1000 {
            format!("{} KB", (size as f64 / 1000.0).round() as u64)
        } else if size < 1000 * 1000 * 1000 {
            format!("{:.1} MB", size as f64 / (1000.0 * 1000.0))
        } else {
            format!("{:.2} GB", size as f64 / (1000.0 * 1000.0 * 1000.0))
        }
    } else {
        // Base-2 (binary)
        if size < 1024 {
            format!("{} B", (size as f64 / 1024.0).round() as u64)
        } else if size < 1024 * 1024 {
            format!("{} KB", (size as f64 / 1024.0).round() as u64)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}


// 写入 JSON 文件
pub fn write_json_file<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Error> {
    let json_str = serde_json::to_string_pretty(data)?;
    let path_str = path.as_os_str().to_string_lossy().to_string();
    fs::write(path, json_str.as_bytes())
        .with_context(|| format!("failed to save file \"{path_str}\""))
}

// 读取 JSON 文件
pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let contents = fs::read_to_string(path)?;
    let value = serde_json::from_str(&contents)?;
    Ok(value)
}

pub fn load_font_metadata(path: String) -> Result<FontMeta, String> {
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    let face = font_kit::handle::Handle::from_memory(data.into(), 0);
    match face.load() {
        Ok(loaded_face) => Ok(FontMeta {
            postscript_name: loaded_face.postscript_name().unwrap_or_default(),
            family: loaded_face.family_name(),
            full_name: loaded_face.full_name(),
        }),
        Err(e) => Err(format!("加载字体失败: {}", e)),
    }
}
