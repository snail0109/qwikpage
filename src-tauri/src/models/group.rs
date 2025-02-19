use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::{self, ErrorKind};
use uuid::Uuid;

use crate::commands::project::get_project_list_new;
use crate::utils::get_app_root_dir;

use super::project::ProjectSummary;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub projects: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupConfig {
    pub groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupDetail {
    pub id: String,
    pub name: String,
    pub projects: Option<Vec<ProjectSummary>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupList {
    pub groups: Vec<GroupDetail>,
}

pub enum UpdateOption {
    Add,
    Remove,
}

impl GroupConfig {
    /// 从文件加载配置
    pub fn load() -> io::Result<Self> {
        let path = get_app_root_dir().join("group.json");
        if !path.exists() {
            let config = GroupConfig { groups: vec![] };
            config.save()?;
        }

        match fs::read_to_string(path) {
            Ok(data) => {
                let config: GroupConfig = serde_json::from_str(&data).unwrap();
                Ok(config)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // 如果文件不存在，返回一个空配置
                Ok(GroupConfig { groups: vec![] })
            }
            Err(e) => Err(e),
        }
    }

    /// 将配置保存到文件
    pub fn save(&self) -> io::Result<()> {
        let path = get_app_root_dir().join("group.json");
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json)
    }

    /// 添加一个新分组
    pub fn add_group(&mut self, name: String) -> Result<String, Error> {
        let id = Uuid::new_v4().to_string();
        let group = Group {
            id: id.clone(),
            name,
            projects: None,
        };
        self.groups.push(group);
        info!("group added: {:?}", self.groups);
        self.save()?;
        Ok(id)
    }

    /// 删除一个分组
    pub fn delete_group(&mut self, id: &str) -> Result<bool, Error> {
        let original_len = self.groups.len();
        self.groups.retain(|group| group.id != id);
        self.save()?;
        Ok(original_len != self.groups.len())
    }

    /// 更新一个分组
    pub fn update_group(&mut self, id: &str, name: Option<String>) -> Result<bool, Error> {
        if let Some(group) = self.groups.iter_mut().find(|group| group.id == id) {
            if let Some(new_name) = name {
                group.name = new_name;
            }
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // 查询所有分组并遍历分组下的项目
    pub fn get_project_details(&self, keyword: Option<String>) -> Result<GroupList, Error> {
        let projects = get_project_list_new(keyword).unwrap();
        // 收集所有已被分配的项目ID
        let mut assigned_project_ids = HashSet::new();

        let mut group_list = Vec::new();

        // 遍历分组下的项目，从projects 获详情，组装GroupList 数据
        for group in &self.groups {
            let mut projects_in_group = Vec::new();
            if let Some(project_ids) = &group.projects {
                for project_id in project_ids {
                    assigned_project_ids.insert(project_id.to_string());
                    if let Some(project) = projects
                        .iter()
                        .find(|project| project.id == project_id.to_string())
                    {
                        projects_in_group.push(project.clone());
                    }
                }
            }
            group_list.push(GroupDetail {
                id: group.id.clone(),
                name: group.name.clone(),
                projects: Some(projects_in_group),
            });
        }

        let default_projects: Vec<_> = projects
            .iter()
            .filter(|p| !assigned_project_ids.contains(&p.id))
            .cloned()
            .collect();

        group_list.push(GroupDetail {
            id: String::from("-1"),        // 默认分组ID
            name: String::from("Default"), // 默认分组名称
            projects: Some(default_projects),
        });

        Ok(GroupList { groups: group_list })
    }

    pub fn update_group_project(
        &mut self,
        id: String,
        project_id: String,
        option: UpdateOption,
    ) -> Result<bool, Error> {
        // 查找目标分组
        let group = match self.groups.iter_mut().find(|group| group.id == id) {
            Some(group) => group,
            None => return Ok(false), // 如果分组不存在，直接返回
        };
        let projects = group.projects.get_or_insert_with(Vec::new);

        match option {
            UpdateOption::Add => {
                // 如果项目ID尚未存在，则添加
                if !projects.contains(&project_id.to_string()) {
                    projects.push(project_id.to_string());
                    info!("Project {} added to group {}", project_id, id);
                }
            }
            UpdateOption::Remove => {
                // 移除指定的项目ID
                let original_len = projects.len();
                projects.retain(|p| *p != project_id.to_string());
                if projects.len() < original_len {
                    info!("Project {} removed from group {}", project_id, id);
                }
            }
        }
        self.save()?;
        Ok(true)
    }
}
