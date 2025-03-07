use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::storage::get_app_root_resource_dir;
use crate::models::page::PageContent;
use futures::future::BoxFuture;
use log::info;
use reqwest;
use serde_json::Value;
use tokio::fs as async_fs;

use crate::error::{CmdError, Result};

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

/// 创建目录（如果不存在）
pub async fn create_dir_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        async_fs::create_dir_all(path)
            .await
            .map_err(|e| CmdError::Io(e))?;
    }
    Ok(())
}

/// 写入文件
pub async fn write_file(path: PathBuf, content: &str) -> Result<()> {
    async_fs::write(&path, content)
        .await
        .map_err(|e| CmdError::Io(e))
}

/// 读取文件（如果存在）
pub async fn read_file_if_exists(path: &Path) -> Result<String> {
    match async_fs::read_to_string(path).await {
        Ok(c) => Ok(c),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(CmdError::Io(e)),
    }
}

/// 更新配置文件
pub async fn update_config_file(path: PathBuf, marker: &str, content: &str) -> Result<()> {
    let original = read_file_if_exists(&path).await?;
    let updated = original.replace(marker, &format!("{}\n", content));
    write_file(path, &updated).await
}

/// 递归复制目录
pub fn copy_directory_recursive(
    source_dir: &PathBuf,
    target_dir: &PathBuf,
) -> BoxFuture<'static, Result<()>> {
    let source_dir = source_dir.clone();
    let target_dir = target_dir.clone();

    Box::pin(async move {
        // 确保目标目录存在
        async_fs::create_dir_all(&target_dir)
            .await
            .map_err(|e| CmdError::Io(e))?;

        // 读取源目录内容
        let mut entries = async_fs::read_dir(source_dir)
            .await
            .map_err(|e| CmdError::Io(e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| CmdError::Io(e))?
        {
            let source_path = entry.path();
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|_| CmdError::Other(format!("无效的文件名: {:?}", source_path)))?;
            let target_path = &target_dir.join(file_name);

            if source_path.is_dir() {
                // 递归复制子目录
                copy_directory_recursive(&source_path, &target_path).await?;
            } else {
                // 复制文件
                async_fs::copy(&source_path, &target_path)
                    .await
                    .map_err(|e| CmdError::Io(e))?;
            }
        }

        Ok(())
    })
}

/// 导出资源文件
pub async fn export_resources(
    project_id: &str,
    code_dir: &PathBuf,
    public_dir_name: &str,
) -> Result<()> {
    // 获取项目资源目录
    let prj_res_dir = get_app_root_resource_dir().join(project_id);

    if !prj_res_dir.exists() {
        info!("资源目录不存在: {:?}", prj_res_dir);
        return Ok(());
    }

    // 目标目录：code_dir 的 public 子目录
    let public_dir = code_dir.join(public_dir_name);

    // 确保目标目录存在
    async_fs::create_dir_all(&public_dir)
        .await
        .map_err(|e| CmdError::Io(e))?;

    // 读取资源目录内容
    let mut entries = async_fs::read_dir(&prj_res_dir)
        .await
        .map_err(|e| CmdError::Io(e))?;

    // 复制目录内容
    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| CmdError::Io(e))?
    {
        let source_path = entry.path();
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| CmdError::Other(format!("无效的文件名: {:?}", source_path)))?;
        let target_path = public_dir.join(file_name);

        if source_path.is_dir() {
            // 如果是目录，递归复制
            copy_directory_recursive(&source_path, &target_path).await?;
        } else {
            // 如果是文件，直接复制
            async_fs::copy(&source_path, &target_path)
                .await
                .map_err(|e| CmdError::Io(e))?;
        }
    }

    Ok(())
}

/// 处理页面数据，替换资源路径
pub fn process_page_data(page_data: &str, project_id: &str) -> Result<PageContent> {
    info!("process_page_data...");
    let resource_path = get_app_root_resource_dir().join(project_id);
    let resource_path_str = resource_path.to_str().unwrap_or("");

    // 如果 page_data 为空，返回一个空的PageContent
    if page_data.is_empty() {
        return Ok(PageContent {
            elements: vec![],
            elements_map: HashMap::new(),
            apis: HashMap::new(),
            interceptor: None,
        });
    }

    // 替换资源路径
    let processed_data = page_data.replace(resource_path_str, "/");

    // 解析JSON
    serde_json::from_str(&processed_data).map_err(|e| CmdError::Json(e))
}

/// 替换模板中的变量
pub fn replace_template(template: &str, replacements: &HashMap<&str, impl AsRef<str>>) -> String {
    let mut result = template.to_string();
    for (key, value) in replacements {
        result = result.replace(&format!("{{{}}}", key), value.as_ref());
    }
    result
}

/// 下载并解压模板文件
pub async fn download_template(template_url: &str, output_dir: &PathBuf) -> Result<()> {
    // 下载代码模板
    let template_path = output_dir.join("template.zip");

    info!("下载代码模板: {}", template_url);
    let response = reqwest::get(template_url)
        .await
        .map_err(|e| CmdError::DownloadError(e.to_string()))?;

    let content = response
        .bytes()
        .await
        .map_err(|e| CmdError::DownloadError(e.to_string()))?;

    info!("保存zip文件到: {:?}", template_path);
    async_fs::write(&template_path, content)
        .await
        .map_err(|e| CmdError::Io(e))?;

    info!("解压文件");
    let file = std::fs::File::open(&template_path).map_err(|e| CmdError::Io(e))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| CmdError::Other(format!("读取zip文件失败: {}", e)))?;

    extract_archive(&mut archive, output_dir)?;

    info!("删除zip文件");
    std::fs::remove_file(&template_path).map_err(|e| CmdError::Io(e))?;

    // mac 下会生成 __MACOSX 文件
    #[cfg(target_os = "macos")]
    remove_macosx_folder(&template_path)?;

    Ok(())
}

/// 解压文件
fn extract_archive(
    archive: &mut zip::ZipArchive<std::fs::File>,
    output_dir: &PathBuf,
) -> Result<()> {
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| CmdError::Other(format!("访问zip文件条目失败: {}", e)))?;

        let outpath = output_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| CmdError::Io(e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| CmdError::Io(e))?;
                }
            }

            let mut outfile = std::fs::File::create(&outpath).map_err(|e| CmdError::Io(e))?;

            std::io::copy(&mut file, &mut outfile).map_err(|e| CmdError::Io(e))?;
        }
    }

    Ok(())
}

/// 删除macOS特有的文件夹
#[cfg(target_os = "macos")]
fn remove_macosx_folder(template_path: &PathBuf) -> Result<()> {
    let macosx_path = template_path.with_file_name("__MACOSX");

    if macosx_path.exists() {
        std::fs::remove_dir_all(macosx_path).map_err(|e| CmdError::Io(e))?;
    }

    Ok(())
}
