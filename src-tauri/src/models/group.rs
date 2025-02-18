use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use uuid::Uuid;
use anyhow::Error;

use crate::utils::get_app_root_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub projects: Option<Vec<Uuid>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub groups: Vec<Group>,
}

impl ConfigFile {
    /// 从文件加载配置
    pub fn load() -> io::Result<Self> {
        let path = get_app_root_dir().join("group.json");
        match fs::read_to_string(path) {
            Ok(data) => {
                let config: ConfigFile = serde_json::from_str(&data).unwrap();
                Ok(config)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // 如果文件不存在，返回一个空配置
                Ok(ConfigFile { groups: vec![] })
            }
            Err(e) => Err(e),
        }
    }

    /// 将配置保存到文件
    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json)
    }

    /// 添加一个新分组
    pub fn add_group(&mut self, name: String) -> Result<String, Error> {
        let id = Uuid::new_v4().to_string();
        let group = Group { id: id.clone(), name, projects: None };
        self.groups.push(group);
        Ok(id)
    }

    /// 删除一个分组
    pub fn delete_group(&mut self, id: &str) -> Result<bool, Error> {
        let original_len = self.groups.len();
        self.groups.retain(|group| group.id != id);
        Ok(original_len != self.groups.len())
    }

    /// 更新一个分组
    pub fn update_group(&mut self, id: &str, name: Option<String>, projects: Option<Option<Vec<Uuid>>>) -> Result<bool, Error> {
        if let Some(group) = self.groups.iter_mut().find(|group| group.id == id) {
            if let Some(new_name) = name {
                group.name = new_name;
            }
            if let Some(new_projects) = projects {
                group.projects = new_projects;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 查询所有分组并遍历分组下的项目
    // pub fn get_all_groups_with_project_details<F>(&self, project_detail_fn: F) -> Vec<(Group, Vec<Option<String>>)>
    // where
    //     F: Fn(Uuid) -> Option<String>,
    // {
    //     self.groups
    //         .iter()
    //         .map(|group| {
    //             let project_details = group
    //                 .projects
    //                 .as_ref()
    //                 .map(|projects| projects.iter().map(|&project_id| project_detail_fn(project_id)).collect())
    //                 .unwrap_or_else(Vec::new);
    //             (group.clone(), project_details)
    //         })
    //         .collect()
    // }



}
