use std::{fs, path::PathBuf};

use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};

use crate::utils::get_app_root_resource_dir;
use tokio::fs::read_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceQueryParams {
    pub project_id: String,
    pub resouce_type: String,
    pub resouce_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
    pub res_type: String,
}

pub struct ResourceConfig {}

impl ResourceConfig {
    /// 从文件加载配置
    pub async fn load(params: ResourceQueryParams) -> Result<Vec<ResourceInfo>, Error> {
        // 项目资源路径
        let prj_res_dir = get_app_root_resource_dir().join(params.project_id);
        if !prj_res_dir.exists() {
            info!("create project resource dir: {:?}", prj_res_dir);
            fs::create_dir_all(&prj_res_dir).expect("failed to create project resource dir");
        }

        // 资源类型跟路径，也就是默认分组
        let res_type_dir = prj_res_dir.join(params.resouce_type);
        if !res_type_dir.exists() {
            info!("create resource type dir: {:?}", res_type_dir);
            fs::create_dir_all(&res_type_dir).expect("failed to create resource type dir");
        }
        // 资源分组路径
        let mut res_group_dir = res_type_dir.clone();
        // 子分组
        if let Some(res_group) = params.resouce_group {
            res_group_dir = res_type_dir.join(res_group);
            if !res_group_dir.exists() {
                info!("create resource group dir: {:?}", res_group_dir);
                fs::create_dir_all(&res_group_dir).expect("failed to create resource group dir");
            }
        }
        // 遍历 res_group_dir 下的所有文件，返回文件名和文件路径的列表
        let mut result = read_dir(res_group_dir).await?;
        let mut resources: Vec<ResourceInfo> = vec![];
        while let Ok(Some(entry)) = result.next_entry().await {
            if entry.path().is_dir() {
                let path_str = process_path(entry.path());
                resources.push(ResourceInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: path_str,
                    // 判断你是文件还是目录
                    res_type: String::from("dir"),
                });
            } else {
                // 排除 .DS_Store 文件
                if entry.file_name() == ".DS_Store" {
                    continue;
                }
                let path_str = process_path(entry.path());
                resources.push(ResourceInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: path_str,
                    res_type: String::from("file"),
                });
            }
        }
        Ok(resources)
    }

}


fn process_path(p: PathBuf) -> String {
    #[cfg(target_os = "windows")]
    {
        fix_windows_paths(&p)
    }
    #[cfg(not(target_os = "windows"))]
    {
        p.to_string_lossy().to_string()
    }
}

#[cfg(target_os = "windows")]
fn fix_windows_paths(p: &PathBuf) -> String {
    use std::path::Component;

    // 1. Remove UNC prefix (if exists)
    let mut components = p.components();
    if let Some(Component::Prefix(prefix)) = components.next() {
        if prefix.kind().is_verbatim() {
            components = p.strip_prefix(prefix.as_os_str()).unwrap().components();
        }
    }

    // 2. Remove the drive letter
    let path_without_drive = components.as_path().to_string_lossy();

    // 3. Convert backslashes to forward slashes
    let final_path = path_without_drive.replace("\\", "/");

    final_path
}
