use anyhow::Error;
use log::info;
use std::collections::HashSet;
use std::fs;
use std::io::{self, ErrorKind};
use uuid::Uuid;

use crate::types::group::{Group, GroupConfig, GroupDetail, GroupList};
use crate::types::project::Project;
use crate::utils::datetime::get_current_time;
use crate::utils::dirs::get_config_path;

fn default_group() -> Group {
    Group {
        id: "-1".to_string(),
        name: "默认分组".to_string(),
        projects: None,
        created_at: None,
        updated_at: None,
    }
}

impl GroupConfig {
    /// 从文件加载配置
    pub fn load() -> io::Result<Self> {
        let path = get_config_path().join("group.json");
        if !path.exists() {
            let mut def_group = default_group();
            def_group.created_at = Some(get_current_time());
            def_group.updated_at = Some(get_current_time());
            let config = GroupConfig {
                groups: vec![def_group],
            };
            config.save()?;
        }

        match fs::read_to_string(path) {
            Ok(data) => {
                let config: GroupConfig = serde_json::from_str(&data)?;
                Ok(config)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // 如果文件不存在，返回默认分组
                let mut def_group = default_group();
                def_group.created_at = Some(get_current_time());
                def_group.updated_at = Some(get_current_time());
                Ok(GroupConfig {
                    groups: vec![def_group],
                })
            }
            Err(e) => Err(e),
        }
    }

    /// 将配置保存到文件
    pub fn save(&self) -> io::Result<()> {
        let path = get_config_path().join("group.json");
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
            created_at: Some(get_current_time()),
            updated_at: Some(get_current_time()),
        };
        self.groups.push(group);
        info!("group added: {:?}", self.groups);
        self.save()?;
        Ok(id)
    }

    /// 删除一个分组
    pub fn delete_group(&mut self, id: &str) -> Result<bool, Error> {
        // 默认分组，不允许删除
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
                group.updated_at = Some(get_current_time());
            }
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // 查询所有分组并遍历分组下的项目
    pub fn get_project_details(&self, keyword: Option<String>) -> Result<GroupList, Error> {
        let projects = Project::get_project_list_inner(keyword).unwrap();
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
                created_at: group.created_at.clone().unwrap_or_default(),
                updated_at: group.updated_at.clone().unwrap_or_default(),
                projects: Some(projects_in_group),
            });
        }

        // 处理未分配的项目
        let unassigned_projects: Vec<_> = projects
            .iter()
            .filter(|p| !assigned_project_ids.contains(&p.id))
            .cloned()
            .collect();

        if !unassigned_projects.is_empty() {
            // 遍历 group_list  如果 id wei -1，则将unassigned_projects 合并到它的 projects 里面，数据做合并不是覆盖
            for group in &mut group_list {
                if group.id == "-1".to_string() {
                    if let Some(projects) = &mut group.projects {
                        projects.extend(unassigned_projects.iter().cloned());
                    } else {
                        group.projects = Some(unassigned_projects.clone());
                    }
                }
            }
        }

        // 按照创建时间对 group_list 进行排序
        group_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(GroupList { groups: group_list })
    }

    pub fn add_group_project(&mut self, id: String, project_id: String) -> Result<bool, Error> {
        // 查找目标分组
        let group = match self.groups.iter_mut().find(|group| group.id == id) {
            Some(group) => group,
            None => return Ok(false), // 如果分组不存在，直接返回
        };
        let projects = group.projects.get_or_insert_with(Vec::new);

        // 如果项目ID尚未存在，则添加
        if !projects.contains(&project_id.to_string()) {
            projects.push(project_id.to_string());
            info!("Project {} added to group {}", project_id, id);
        }
        self.save()?;
        Ok(true)
    }

    pub fn remove_project_from_group(
        &mut self,
        group_id: String,
        project_id: String,
    ) -> Result<(), Error> {
        // group_id 如果是 -1 则是默认分组，不允许删除
        // if group_id == "-1" {
        //     return Err(anyhow::anyhow!("默认分组，不允许删除"));
        // }
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(projects) = &mut group.projects {
                if projects.contains(&project_id) {
                    projects.retain(|id| id != &project_id);
                    self.save()?;
                    info!("Project {} removed from group {}", project_id, group_id);
                    return Ok(());
                }
            }
        }
        // 兼容处理手工复制的项目没有直接加入分组的情况
       Ok(())
    }
}
