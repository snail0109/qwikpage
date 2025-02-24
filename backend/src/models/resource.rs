use std::fs;

use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};

use crate::utils::{format_system_time, get_app_root_resource_dir};
use tokio::fs::read_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceQueryParams {
    pub project_id: String,
    pub resouce_type: String,
    pub resouce_group: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
    pub res_type: String,
    pub last_modified_time: String,
}

pub struct ResourceConfig {}

impl ResourceConfig {
    // 查询接口，根据项目 ID、资源类型、资源分组、关键字查询资源
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
            // 支持 keyword 过滤
            if let Some(keyword) = &params.keyword {
                if !entry.file_name().to_string_lossy().contains(keyword) {
                    continue;
                }
            }
            if entry.path().is_dir() {
                resources.push(ResourceInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    // 判断你是文件还是目录
                    res_type: String::from("dir"),
                    // 返回目录操作时间
                    last_modified_time: format_system_time(entry.path().metadata().unwrap().modified().unwrap()),
                });
            } else {
                // 排除 .DS_Store 文件
                if entry.file_name() == ".DS_Store" {
                    continue;
                }
                resources.push(ResourceInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    // 文件就返回具体的文件类型
                    res_type: entry.path().extension().unwrap().to_string_lossy().to_string(),
                    // 返回文件操作时间, 转换成 YYYY/MM/DD 格式
                    last_modified_time: format_system_time(entry.path().metadata().unwrap().modified().unwrap()),
                });
            }
        }
        Ok(resources)
    }

    pub async fn save() {

    }

    pub async fn add_resource() {

    }

    pub async fn delete_resource() {

    }

    pub async fn update_resource() {

    }


    pub async fn upload_resource() {
        
    }



}