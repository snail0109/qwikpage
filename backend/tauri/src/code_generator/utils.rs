use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::storage::config::Config;
use futures::future::BoxFuture;
use log::info;
use reqwest;
use serde_json::Value;
use tokio::fs as async_fs;

use crate::error::{CommonError, Result};
use code_core::types::page::PageContent;

// 将 JSON 值转换为 JavaScript 表示的字符串
#[allow(unused)]
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
#[allow(unused)]
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// 创建目录（如果不存在）
#[allow(unused)]
pub async fn create_dir_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        async_fs::create_dir_all(path)
            .await
            .map_err(|e| CommonError::Io(e))?;
    }
    Ok(())
}

/// 写入文件
#[allow(unused)]
pub async fn write_file(path: PathBuf, content: &str) -> Result<()> {
    async_fs::write(&path, content)
        .await
        .map_err(|e| CommonError::Io(e))
}

/// 读取文件（如果存在）
#[allow(unused)]
pub async fn read_file_if_exists(path: &Path) -> Result<String> {
    match async_fs::read_to_string(path).await {
        Ok(c) => Ok(c),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(CommonError::Io(e)),
    }
}

/// 更新配置文件
#[allow(unused)]
pub async fn update_config_file(path: PathBuf, marker: &str, content: &str) -> Result<()> {
    let original = read_file_if_exists(&path).await?;
    let updated = original.replace(marker, &format!("{}\n", content));
    write_file(path, &updated).await
}

/// 递归复制目录
#[allow(unused)]
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
            .map_err(|e| CommonError::Io(e))?;

        // 读取源目录内容
        let mut entries = async_fs::read_dir(source_dir)
            .await
            .map_err(|e| CommonError::Io(e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| CommonError::Io(e))? {
            let source_path = entry.path();
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|_| CommonError::Other(format!("无效的文件名: {:?}", source_path)))?;
            let target_path = &target_dir.join(file_name);

            if source_path.is_dir() {
                // 递归复制子目录
                copy_directory_recursive(&source_path, &target_path).await?;
            } else {
                // 复制文件
                async_fs::copy(&source_path, &target_path)
                    .await
                    .map_err(|e| CommonError::Io(e))?;
            }
        }

        Ok(())
    })
}

/// 导出资源文件
#[allow(unused)]
pub async fn export_resources(
    project_id: &str,
    code_dir: &PathBuf,
    public_dir_name: &str,
) -> Result<()> {
    // 获取项目资源目录
    let config_path = Config::global().preferences().get_project_path();
    let prj_res_dir = config_path.join(&project_id).join("resources");

    log::info!("config_path: {:?}, {:?}", config_path, prj_res_dir);

    if !prj_res_dir.exists() {
        info!("资源目录不存在: {:?}", prj_res_dir);
        return Ok(());
    }

    // 目标目录：code_dir 的 public 子目录
    let public_dir = code_dir.join(public_dir_name);

    // 确保目标目录存在
    async_fs::create_dir_all(&public_dir)
        .await
        .map_err(|e| CommonError::Io(e))?;

    // 复制资源，应用特殊规则
    copy_resources_with_rules(&prj_res_dir, &public_dir, None).await?;

    Ok(())
}

/// 辅助函数，用于递归复制资源并应用特殊规则
async fn copy_resources_with_rules(
    source_dir: &PathBuf, 
    target_dir: &PathBuf,
    parent_dir_name: Option<&str>
) -> Result<()> {
    // 读取源目录内容
    let mut entries = async_fs::read_dir(&source_dir)
        .await
        .map_err(|e| CommonError::Io(e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| CommonError::Io(e))? {
        let source_path = entry.path();
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| CommonError::Other(format!("无效的文件名: {:?}", source_path)))?;
        
        // 应用规则1：将"默认分组"重命名为"defaultGroup"
        let target_file_name = if file_name == "默认分组" {
            "defaultGroup".to_string()
        } else {
            file_name.clone()
        };
        
        // 应用规则2：如果是"默认分组"下的"main"目录，则跳过
        if parent_dir_name == Some("默认分组") && file_name == "main" {
            log::info!("跳过'默认分组'下的'main'目录: {:?}", source_path);
            continue;
        }
        
        let target_path = target_dir.join(&target_file_name);

        if source_path.is_dir() {
            // 为子目录创建目标目录
            async_fs::create_dir_all(&target_path)
                .await
                .map_err(|e| CommonError::Io(e))?;
            
            // 使用 Box::pin 处理递归异步调用
            Box::pin(copy_resources_with_rules(&source_path, &target_path, Some(&file_name))).await?;
        } else {
            // 如果是文件，直接复制
            async_fs::copy(&source_path, &target_path)
                .await
                .map_err(|e| CommonError::Io(e))?;
        }
    }

    Ok(())
}
/// 处理页面数据，替换资源路径
#[allow(unused)]
pub fn process_page_data(page_data: &str, project_id: &str) -> Result<PageContent> {
    info!("process_page_data...");
    let resource_path = Config::global()
        .preferences()
        .get_project_path()
        .join(project_id);
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
    serde_json::from_str(&processed_data).map_err(|e| CommonError::Json(e))
}

/// 替换模板中的变量
#[allow(unused)]
pub fn replace_template(template: &str, replacements: &HashMap<&str, impl AsRef<str>>) -> String {
    let mut result = template.to_string();
    for (key, value) in replacements {
        result = result.replace(&format!("{{{}}}", key), value.as_ref());
    }
    result
}

/// 下载并解压模板文件
#[allow(unused)]
pub async fn download_template(template_url: &str, output_dir: &PathBuf) -> Result<()> {
    // 下载代码模板
    let template_path = output_dir.join("template.zip");

    info!("下载代码模板: {}", template_url);
    let response = reqwest::get(template_url)
        .await
        .map_err(|e| CommonError::DownloadError(e.to_string()))?;

    let content = response
        .bytes()
        .await
        .map_err(|e| CommonError::DownloadError(e.to_string()))?;

    info!("保存zip文件到: {:?}", template_path);
    async_fs::write(&template_path, content)
        .await
        .map_err(|e| CommonError::Io(e))?;

    info!("解压文件");
    let file = std::fs::File::open(&template_path).map_err(|e| CommonError::Io(e))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| CommonError::Other(format!("读取zip文件失败: {}", e)))?;

    extract_archive(&mut archive, output_dir)?;

    info!("删除zip文件");
    std::fs::remove_file(&template_path).map_err(|e| CommonError::Io(e))?;

    // mac 下会生成 __MACOSX 文件
    #[cfg(target_os = "macos")]
    remove_macosx_folder(&template_path)?;

    Ok(())
}

/// 解压文件
#[allow(unused)]
fn extract_archive(
    archive: &mut zip::ZipArchive<std::fs::File>,
    output_dir: &PathBuf,
) -> Result<()> {
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| CommonError::Other(format!("访问zip文件条目失败: {}", e)))?;

        let outpath = output_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| CommonError::Io(e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| CommonError::Io(e))?;
                }
            }

            let mut outfile = std::fs::File::create(&outpath).map_err(|e| CommonError::Io(e))?;

            std::io::copy(&mut file, &mut outfile).map_err(|e| CommonError::Io(e))?;
        }
    }

    Ok(())
}

/// 删除macOS特有的文件夹
#[cfg(target_os = "macos")]
fn remove_macosx_folder(template_path: &PathBuf) -> Result<()> {
    let macosx_path = template_path.with_file_name("__MACOSX");

    if macosx_path.exists() {
        std::fs::remove_dir_all(macosx_path).map_err(|e| CommonError::Io(e))?;
    }

    Ok(())
}