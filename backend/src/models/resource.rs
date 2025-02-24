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
    pub keyword: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperResourceGroupParams {
    pub project_id: String,
    pub resouce_type: String,
    pub group_name: String,
    pub new_group_name: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UploadParams {
    pub project_id: String,
    pub resouce_type: String,
    pub group_name: String,
    pub file_list: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
    pub res_type: String,
    pub last_modified_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceGroupInfo {
    pub name: String,
    pub path: String,
    pub last_modified_time: String,
    pub resources: Vec<ResourceInfo>,
}

pub struct ResourceConfig {}

impl ResourceConfig {
    // 查询接口，根据项目 ID、资源类型、资源分组、关键字查询资源
    pub async fn load(params: ResourceQueryParams) -> Result<Vec<ResourceGroupInfo>, Error> {
        // 资源分组根路径
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resouce_type);
        // 查看资源目录下是否存在默认分组没有则创建
        check_default_group_dir(&res_root_dir);
        
        // 遍历 res_root_dir 下的所有目录以及内部文件，返回文件名和文件路径的列表
        let mut result = read_dir(res_root_dir).await?;
        let mut resource_groups : Vec<ResourceGroupInfo> = vec![];
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
                        // 判断你是文件还是目录
                        res_type: String::from("file"),
                        // 返回目录操作时间
                        last_modified_time: format_system_time(dir_entry.path().metadata().unwrap().modified().unwrap()),
                    });
                }
                resource_groups.push(ResourceGroupInfo {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    last_modified_time: format_system_time(entry.path().metadata().unwrap().modified().unwrap()),
                    resources,
                });
            }
        }

        Ok(resource_groups)
    }

    pub async fn save() {

    }

    pub async fn add_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resouce_type);
        let group_dir = res_root_dir.join(&params.group_name);
        if !group_dir.exists() {
            fs::create_dir_all(&group_dir).expect("failed to create resource group dir");
        }
        Ok(true)
    }

    pub async fn update_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resouce_type);
        let old_group_dir = res_root_dir.join(&params.group_name);
        let new_group_dir = res_root_dir.join(params.new_group_name.as_ref().ok_or_else(|| Error::msg("new_group_name is None"))?);
        fs::rename(old_group_dir, new_group_dir).expect("failed to rename resource group dir");
        Ok(true)
    }

    pub async fn delete_resource_group(params: OperResourceGroupParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resouce_type);
        let group_dir = res_root_dir.join(&params.group_name);
        fs::remove_dir_all(group_dir).expect("failed to remove resource group dir");
        Ok(true)
    }


    // 将本地一个资源复制到本地指定分组
    pub async fn import_resources(params: UploadParams) -> Result<bool, Error> {
        let res_root_dir = get_res_root_dir(&params.project_id, &params.resouce_type);
        let group_dir = res_root_dir.join(&params.group_name);
        if !group_dir.exists() {
            fs::create_dir_all(&group_dir).expect("failed to create resource group dir");
        }
        for file in &params.file_list {
            let file_path = std::path::Path::new(file);
            let file_name = file_path.file_name().unwrap();
            let new_file_path = group_dir.join(file_name);
            fs::copy(file_path, new_file_path).expect("failed to copy file");
        }
        Ok(true)
    }

}

// 资源分组根路径
fn get_res_root_dir(project_id: &String , resouce_type : &String) -> std::path::PathBuf {
    // 项目资源路径
    let prj_res_dir = get_app_root_resource_dir().join(project_id);
    if !prj_res_dir.exists() {
        info!("create project resource dir: {:?}", prj_res_dir);
        fs::create_dir_all(&prj_res_dir).expect("failed to create project resource dir");
    }

    // 资源类型根路径
    let res_root_dir = prj_res_dir.join(resouce_type);
    if !res_root_dir.exists() {
        info!("create resource type dir: {:?}", res_root_dir);
        fs::create_dir_all(&res_root_dir).expect("failed to create resource type dir");
    }
    res_root_dir
}


fn check_default_group_dir(res_root_dir: &std::path::PathBuf) {
    let def_res_group = res_root_dir.join("默认分组");
    if !def_res_group.exists() {
        info!("create resource default group dir: {:?}", def_res_group);
        fs::create_dir_all(&def_res_group).expect("failed to create resource group dir");
    }
}
