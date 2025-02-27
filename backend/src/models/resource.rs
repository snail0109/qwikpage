use anyhow::Error;
use log::info;
use sanitize_filename::sanitize;
use serde::{Deserialize, Serialize};

use crate::utils::{format_system_time, get_app_root_resource_dir};
use futures::future::join_all;
use std::path::{Path, PathBuf};
use tokio::fs::{copy, create_dir_all, read_dir, remove_dir_all, rename};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Img,
    Font,
    Js,
    Attachment,
    Other,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Img => "img",
            ResourceType::Font => "font",
            ResourceType::Js => "js",
            ResourceType::Attachment => "attachment",
            ResourceType::Other => "other",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceQueryParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub keyword: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperResourceGroupParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub new_group_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadParams {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub file_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub last_modified_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceGroupInfo {
    pub name: String,
    pub path: String,
    pub last_modified_time: String,
    pub resources: Vec<ResourceInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameResource {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub resource_name: String,
    pub new_resource_name: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DeleteResource {
    pub project_id: String,
    pub resource_type: ResourceType,
    pub group_name: String,
    pub resource_name: String,
}

pub struct ResourceConfig {}

impl ResourceConfig {
    // 查询接口，根据项目 ID、资源类型、资源分组、关键字查询资源
    pub async fn load(params: ResourceQueryParams) -> Result<Vec<ResourceGroupInfo>, Error> {
        // 资源分组根路径
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
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
                while let Ok(Some(dir_entry)) = dir_result.next_entry().await {
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
                        last_modified_time: format_system_time(
                            dir_entry.path().metadata().unwrap().modified().unwrap(),
                        ),
                    });
                }
                resource_groups.push(ResourceGroupInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    last_modified_time: format_system_time(
                        entry.path().metadata().unwrap().modified().unwrap(),
                    ),
                    resources,
                });
            }
        }

        Ok(resource_groups)
    }

    pub async fn add_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(sanitize(&params.group_name));
        if !group_dir.exists() {
            info!("创建资源分组目录: {:?}", group_dir);
            create_dir_all(&group_dir)
                .await
                .map_err(|e| Error::new(e).context("Failed to create directory"))?;
        }
        Ok(true)
    }

    pub async fn update_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
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

    pub async fn delete_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        remove_dir_all(group_dir)
            .await
            .map_err(|e| Error::new(e).context("Failed to remove directory"))?;
        Ok(true)
    }

    // 将本地一个资源复制到本地指定分组
    pub async fn import_resources(params: UploadParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        if !group_dir.exists() {
            info!("创建资源分组目录: {:?}", group_dir);
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
                    copy(file_path, new_file_path)
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
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
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
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resource_type).await?;
        let group_dir = res_root_dir.join(&params.group_name);
        let resource_path = group_dir.join(&params.resource_name);
        tokio::fs::remove_file(resource_path)
            .await
            .map_err(|e| Error::new(e).context("Failed to remove file"))?;
        Ok(true)
    }
}

// 资源分组根路径
async fn get_res_root_dir(
    project_id: &String,
    resource_type: &ResourceType,
) -> Result<PathBuf, Error> {
    // 项目资源路径
    let prj_res_dir = get_app_root_resource_dir().join(project_id);
    if !prj_res_dir.exists() {
        info!("创建项目资源目录: {:?}", prj_res_dir);
        tokio::fs::create_dir_all(&prj_res_dir)
            .await
            .map_err(|e| Error::new(e).context("Failed to create directory"))?;
    }

    // 资源类型根路径
    let res_root_dir = prj_res_dir.join(resource_type.as_str());
    if !res_root_dir.exists() {
        info!("create resource type dir: {:?}", res_root_dir);
        tokio::fs::create_dir_all(&res_root_dir)
            .await
            .map_err(|e| Error::new(e).context("Failed to create directory"))?;
    }
    Ok(res_root_dir)
}

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
        info!("创建默认分组目录: {:?}", res_root_dir.join("默认分组"));
        tokio::fs::create_dir_all(&res_root_dir.join("默认分组"))
            .await
            .map_err(|e| Error::new(e).context("Failed to create directory"))?;
    }
    Ok(())
}
