use std::path::PathBuf;

use crate::error::{CommonError, Result};
use crate::manager::preference_manager::PreferencesManager;
use futures::future::BoxFuture;
use log::info;
use tokio::fs as async_fs;

/// 清空目录
#[allow(unused)]
async fn empty_directory(dir_path: &PathBuf) -> std::io::Result<()> {
    if !dir_path.exists() {
        // 如果目录不存在，创建它
        async_fs::create_dir_all(dir_path).await?;
        return Ok(());
    }

    // 读取目录内容
    let mut entries = async_fs::read_dir(dir_path).await?;

    // 删除目录内的所有内容
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            // 递归删除子目录
            async_fs::remove_dir_all(&path).await?;
        } else {
            // 删除文件
            async_fs::remove_file(&path).await?;
        }
    }

    Ok(())
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

/// 清空出码目录
#[allow(unused)]
pub async fn clear_code_dir(code_dir: &PathBuf) -> Result<()> {
    // 清空 public 目录
    let public_dir = code_dir.join("public");
    log::info!("清空 public 目录: {:?}", public_dir);
    empty_directory(&public_dir).await.map_err(|e| {
        log::error!("清空 public 目录失败, {}", e.to_string());
        format!("清空 public 目录失败, {}", e.to_string())
    })?;

    // 清空 src/views 目录
    let views_dir = code_dir.join("src").join("views");
    log::info!("清空 src/views 目录: {:?}", views_dir);
    empty_directory(&views_dir).await.map_err(|e| {
        log::error!("清空 src/views 目录失败, {}", e.to_string());
        format!("清空 src/views 目录失败, {}", e.to_string())
    })?;
    Ok(())
}
/// 导出资源文件
#[allow(unused)]
pub async fn export_resources(
    project_id: &str,
    code_dir: &PathBuf,
    public_dir_name: &str,
) -> Result<()> {
    // 获取项目资源目录
    let config_path = PreferencesManager::get_project_path();
    let prj_res_dir = config_path.join(&project_id).join("resources");

    if !prj_res_dir.exists() {
        info!("项目没有配置静态资源: {:?}", prj_res_dir);
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
    parent_dir_name: Option<&str>,
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
            Box::pin(copy_resources_with_rules(
                &source_path,
                &target_path,
                Some(&file_name),
            ))
            .await?;
        } else {
            // 如果是文件，直接复制
            async_fs::copy(&source_path, &target_path)
                .await
                .map_err(|e| CommonError::Io(e))?;
        }
    }

    Ok(())
}
