use anyhow::{Context, Error};
use chrono::Utc;
use log::info;
use sanitize_filename::sanitize;

use crate::types::resource::{
    DeleteResource, OperResourceGroupParams, RenameResource, ResourceGroupInfo, ResourceInfo,
    ResourceType, UploadParams, UploadResourceParams,
};
use crate::utils::datetime::format_resource_system_time;
use crate::{types::resource::ResourceQueryParams, utils::file::format_file_size};
use futures::future::join_all;
use log::error;
use std::path::{Path, PathBuf};
use tokio::fs::{create_dir_all, read_dir, remove_dir_all, rename};

use super::config::Config;

pub struct ResourceConfig {}

impl ResourceConfig {
    // 查询接口，根据项目 ID、资源类型、资源分组、关键字查询资源
    pub async fn load(params: ResourceQueryParams) -> Result<Vec<ResourceGroupInfo>, Error> {
        // 资源分组根路径
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        // 查看资源目录下是否存在默认分组没有则创建
        check_default_group_dir(&res_root_dir).await?;

        // 遍历 res_root_dir 下的所有目录以及内部文件，返回文件名和文件路径的列表
        let mut result = read_dir(res_root_dir).await?;
        let mut resource_groups: Vec<ResourceGroupInfo> = vec![];
        while let Ok(Some(entry)) = result.next_entry().await {
            if entry.path().is_dir() {
                // 遍历目录下的文件
                let mut resources: Vec<ResourceInfo> = vec![];
                let mut dir_result = read_dir(entry.path()).await?;
                let mut default_group: bool = false;
                while let Ok(Some(dir_entry)) = dir_result.next_entry().await {
                    // 如果存在文件夹 并且叫main 就是默认分组
                    if dir_entry.file_name().to_string_lossy() == "main" {
                        default_group = true;
                        continue;
                    }
                    // 过滤掉隐藏文件
                    if dir_entry.file_name().to_string_lossy().starts_with(".") {
                        continue;
                    }
                    // 根据关键字过滤文件
                    if let Some(keyword) = &params.keyword {
                        if !dir_entry.file_name().to_string_lossy().contains(keyword) {
                            continue;
                        }
                    }
                    resources.push(ResourceInfo {
                        name: dir_entry.file_name().to_string_lossy().to_string(),
                        path: dir_entry.path().to_string_lossy().to_string(),
                        file_type: dir_entry
                            .path()
                            .extension()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        // 返回目录操作时间
                        last_modified_time: format_resource_system_time(
                            dir_entry.path().metadata().unwrap().modified().unwrap(),
                        ),
                        file_size: Some(format_file_size(
                            dir_entry.path().metadata().unwrap().len(),
                        )),
                    });
                }
                // 判断
                resource_groups.push(ResourceGroupInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    last_modified_time: format_resource_system_time(
                        entry.path().metadata().unwrap().modified().unwrap(),
                    ),
                    resources,
                    default_group,
                });
            }
        }

        Ok(resource_groups)
    }

    // 创建本地一个资源分组
    pub async fn add_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(sanitize(&params.group_name));
        if !group_dir.exists() {
            info!("Create resource group directory: {:?}", group_dir);
            create_dir_all(&group_dir)
                .await
                .map_err(|e| Error::new(e).context("Failed to create directory"))?;
        } else {
            error!("The resource group already exists.: {:?}", group_dir);
            return Err(anyhow::anyhow!("The resource group already exists."));
        }
        Ok(true)
    }

    // 更新本地一个资源分组
    pub async fn update_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let old_group_dir = res_root_dir.join(&params.group_name);
        let new_group_dir = res_root_dir.join(
            params
                .new_group_name
                .as_ref()
                .ok_or_else(|| Error::msg("new_group_name is None"))?,
        );
        rename(old_group_dir, new_group_dir)
            .await
            .map_err(|e| Error::new(e).context("Failed to rename directory"))?;
        Ok(true)
    }

    // 删除本地一个资源分组
    pub async fn delete_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        remove_dir_all(group_dir)
            .await
            .map_err(|e| Error::new(e).context("Failed to remove directory"))?;
        Ok(true)
    }

    // 将本地一个资源复制到本地指定分组
    pub async fn import_resources(params: UploadParams) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        if !group_dir.exists() {
            info!("Create resource group directory: {:?}", group_dir);
            create_dir_all(&group_dir)
                .await
                .map_err(|e| Error::new(e).context("Failed to create directory"))?;
        }
        // 使用 futures::future::join_all 来并发处理文件复制
        let copy_futures: Vec<_> = params
            .file_list
            .iter()
            .map(|file| {
                let file_path = Path::new(file);
                let file_name = file_path.file_name().unwrap();
                let new_file_path = group_dir.join(file_name);

                async move {
                    tokio::fs::copy(file_path, new_file_path)
                        .await
                        .map_err(|e| Error::new(e).context("Failed to copy file"))
                }
            })
            .collect();

        // 等待所有的复制操作完成
        let results = join_all(copy_futures).await;

        // 检查结果
        for result in results {
            if let Err(e) = result {
                return Err(e);
            }
        }
        Ok(true)
    }

    // 修改资源名称
    pub async fn rename_resource(params: RenameResource) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        let old_resource_path = group_dir.join(&params.resource_name);
        let new_resource_path = group_dir.join(&params.new_resource_name);
        rename(old_resource_path, new_resource_path)
            .await
            .map_err(|e| Error::new(e).context("Failed to rename file"))?;
        Ok(true)
    }

    // 删除资源
    pub async fn delete_resource(params: DeleteResource) -> Result<bool, Error> {
        let res_root_dir = get_res_type_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        let resource_path = group_dir.join(&params.resource_name);
        tokio::fs::remove_file(resource_path)
            .await
            .map_err(|e| Error::new(e).context("删除资源失败"))?;
        Ok(true)
    }

    // 添加项目临时logo资源
    pub async fn upload_project_resource(params: UploadResourceParams) -> Result<PathBuf, Error> {
        // 构建项目资源目录路径
        let root_dir = Config::global()
            .preferences()
            .get_project_path()
            .join("project_logo");
        log::info!("项目资源目录路径: {:?}", root_dir);
        // temp_res_dir 拼接当前时间戳
        let timestamp = Utc::now().timestamp();
        let temp_res_dir = root_dir.join(timestamp.to_string());

        // 创建目录（如果不存在），使用create_dir_all自动处理已存在的情况
        tokio::fs::create_dir_all(&temp_res_dir)
            .await
            .with_context(|| "创建临时资源目录失败")?;

        // 安全处理文件名，防止路径遍历攻击
        let file_path = Path::new(&params.file_path);
        let file_name = file_path
            .file_name()
            .ok_or_else(|| Error::msg("无效的文件路径"))?;

        // 验证文件名不包含路径分隔符
        let file_name_str = file_name
            .to_str()
            .ok_or_else(|| Error::msg("文件名包含无效字符"))?;

        if file_name_str.contains(|c| c == '/' || c == '\\') {
            return Err(Error::msg("文件名包含非法路径字符."));
        }
        // 构建目标文件路径
        let new_file_path: PathBuf = temp_res_dir.join(file_name);
        info!("创建资源组目录: {:?}", new_file_path);

        // 执行文件复制操作，添加详细错误上下文
        tokio::fs::copy(file_path, new_file_path.clone())
            .await
            .with_context(|| "文件复制失败")?;

        // old_file_path 有值，则删除该路径的文件和上一级目录
        if let Some(old_path) = &params.old_file_path {
            let file_path = Path::new(old_path);
            if let Some(parent_dir) = file_path.parent() {
                // 删除目录
                tokio::fs::remove_dir_all(parent_dir)
                    .await
                    .with_context(|| "删除目录失败")?;
            }
        }

        Ok(new_file_path)
    }

    // 删除项目logo
    pub async fn delete_project_logo(path: String) -> Result<(), Error> {
        let file_path = PathBuf::from(path);
        // 判断 file_path 是否存在
        if !file_path.exists() {
            return Ok(());
        }
        if let Some(parent_dir) = file_path.parent() {
            // 删除目录
            tokio::fs::remove_dir_all(parent_dir)
                .await
                .with_context(|| "删除目录失败")?;
        }
        Ok(())
    }
}

// 资源分组根路径
async fn get_res_type_root_dir(
    project_id: &String,
    resource_type: &ResourceType,
) -> Result<PathBuf, Error> {
    let root_dir = Config::global().preferences().get_project_path();
    let prj_res_dir =
        create_directory_if_not_exists(root_dir.join(project_id).join("resources")).await?;
    let res_root_dir =
        create_directory_if_not_exists(prj_res_dir.join(resource_type.as_str())).await?;
    Ok(res_root_dir)
}

// 创建目录
async fn create_directory_if_not_exists(path: PathBuf) -> Result<PathBuf, Error> {
    if !path.exists() {
        info!("create directory: {:?}", path);
        tokio::fs::create_dir_all(&path)
            .await
            .with_context(|| format!("创建目录失败: {:?}", path))?;
    }
    Ok(path)
}

// 检查默认分组目录是否存在
async fn check_default_group_dir(res_root_dir: &PathBuf) -> Result<(), Error> {
    // 判断根目录下是否有目录，没有则创建默认目录
    let mut dir_count = 0;
    for entry in res_root_dir.read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            dir_count += 1;
        }
    }
    if dir_count == 0 {
        // 默认分组下面再建一个 main 文件夹, 用于查询的时候识别出是否是默认分组
        let def_group = res_root_dir.join("默认分组").join("main");
        info!("创建默认分组目录: {:?}", def_group);
        tokio::fs::create_dir_all(&def_group)
            .await
            .map_err(|e| Error::new(e).context("创建目录失败"))?;
    }
    Ok(())
}
