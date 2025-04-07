use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::RwLock;

use anyhow::{Error, Result};
use log::info;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::manager::preference_manager::PreferencesManager;
use crate::storage::local_storage::LocalStorage;
use crate::types::group::{
    GroupWithProjectDetail, GroupWithProjectList, ProjectGroup, ProjectGroups,
};
use crate::types::project::Project;
use crate::utils::datetime::get_current_time;
use crate::utils::dirs::projects_group_path;

// 顶层数据结构
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProjectsData {
    pub project_groups: HashMap<String, ProjectGroups>,
}

// 项目组管理器
pub struct ProjectGroupManager {
    storage: LocalStorage<ProjectsData>,
    project_groups: RwLock<ProjectsData>,
}

impl ProjectGroupManager {
    // 获取全局单例实例
    pub fn get_instance() -> &'static ProjectGroupManager {
        static INSTANCE: OnceCell<ProjectGroupManager> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let path = projects_group_path();
            let storage = LocalStorage::new(&path.to_string_lossy(), ProjectsData::default);
            let project_group = Self {
                project_groups: RwLock::new(storage.load()),
                storage,
            };
            project_group
        })
    }

    // 创建默认分组
    fn default_group() -> ProjectGroup {
        ProjectGroup {
            id: Uuid::new_v4().to_string(),
            name: "默认分组".to_string(),
            is_default: true,
            projects: None,
            created_at: get_current_time(),
            updated_at: Some(get_current_time()),
        }
    }

    // 获取指定路径的组列表
    fn project_groups<P: AsRef<Path>>(&self, path: P) -> Vec<ProjectGroup> {
        let data = self.project_groups.read().unwrap().clone();
        let path_str = path.as_ref().to_string_lossy().to_string();

           // 检查路径是否存在且有分组信息
           if let Some(pg) = data.project_groups.get(&path_str) {
            if !pg.groups.is_empty() {
                return pg.groups.clone();
            }
        }

        // 如果路径不存在或者没有分组信息，创建默认分组并更新文件
        let default_group = Self::default_group();
        let _ = self.update_project_groups(|data| {
            data.project_groups
                .entry(path_str.clone())
                .or_insert_with(|| ProjectGroups { groups: Vec::new() })
                .groups = vec![default_group.clone()];
        });

        vec![default_group]
    }

    // 更新项目组数据的辅助方法
    fn update_project_groups<F>(&self, updater: F) -> Result<(), Error>
    where
        F: FnOnce(&mut ProjectsData) -> (),
    {
        // 先更新存储
        self.storage.update(|data| {
            updater(data);
        })?;

        // 再更新内存中的数据
        let mut project_groups = self.project_groups.write().unwrap();
        *project_groups = self.storage.load();

        Ok(())
    }

    // 静态方法：添加新组到当前项目路径
    pub fn add_group_to_current_project(name: String, is_default: bool) -> Result<ProjectGroup> {
        let project_path = PreferencesManager::get_project_path();
        let project_group_manager = Self::get_instance();
        let group = ProjectGroup {
            id: Uuid::new_v4().to_string(),
            name,
            is_default,
            created_at: get_current_time(),
            updated_at: Some(get_current_time()),
            projects: None,
        };

        let path_str = project_path.to_string_lossy().to_string();

        project_group_manager.update_project_groups(|data| {
            data.project_groups
                .entry(path_str)
                .or_insert_with(|| ProjectGroups { groups: Vec::new() })
                .groups
                .push(group.clone());
        })?;

        Ok(group)
    }

    // 静态方法：从当前项目路径删除组
    pub fn remove_group_from_current_project(group_id: &str) -> Result<Option<ProjectGroup>> {
        let project_group_manager = Self::get_instance();
        let project_path = PreferencesManager::get_project_path();
        let mut removed = None;
        let path_str = project_path.to_string_lossy().to_string();

        project_group_manager.update_project_groups(|data| {
            if let Some(pg) = data.project_groups.get_mut(&path_str) {
                if let Some(pos) = pg.groups.iter().position(|g| g.id == group_id) {
                    removed = Some(pg.groups.remove(pos));
                }
            }
        })?;

        Ok(removed)
    }

    // 静态方法：更新当前项目路径的组信息
    pub fn update_current_project_group(
        group_id: &str,
        new_name: String,
    ) -> Result<Option<ProjectGroup>> {
        let project_group_manager = Self::get_instance();
        let project_path = PreferencesManager::get_project_path();
        let mut updated = None;
        let path_str = project_path.to_string_lossy().to_string();

        project_group_manager.update_project_groups(|data| {
            if let Some(pg) = data.project_groups.get_mut(&path_str) {
                if let Some(group) = pg.groups.iter_mut().find(|g| g.id == group_id) {
                    group.name = new_name;
                    group.updated_at = Some(get_current_time());
                    updated = Some(group.clone());
                }
            }
        })?;

        Ok(updated)
    }

    // 静态方法：查询当前项目路径的分组及项目
    pub fn get_current_project_details(keyword: Option<String>) -> Result<GroupWithProjectList> {
        let projetc_group_mgr = Self::get_instance();
        let project_path = PreferencesManager::get_project_path();
        let path_str = project_path.to_string_lossy().to_string();
        let cur_project_groups = projetc_group_mgr.project_groups(&path_str);
        let projects = Project::get_project_list_by_option(keyword).unwrap();

        // 收集所有已被分配的项目ID
        let mut assigned_project_ids = HashSet::new();
        let mut group_list = Vec::new();

        // 遍历分组下的项目，从projects 获详情，组装GroupList 数据
        for group in &cur_project_groups {
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
            group_list.push(GroupWithProjectDetail {
                id: group.id.clone(),
                name: group.name.clone(),
                created_at: group.created_at.clone(),
                updated_at: group.updated_at.clone(),
                projects: Some(projects_in_group),
                is_default: group.is_default,
            });
        }

        // 处理未分配的项目
        let unassigned_projects: Vec<_> = projects
            .iter()
            .filter(|p| !assigned_project_ids.contains(&p.id))
            .cloned()
            .collect();

        if !unassigned_projects.is_empty() {
            log::info!("未分组项目({})放置到默认分组", unassigned_projects.len());
            // 遍历 group_list  如果 group.is_default 为 true，则将unassigned_projects 合并到它的 projects 里面
            for group in &mut group_list {
                if group.is_default {
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
        Ok(GroupWithProjectList { groups: group_list })
    }

    // 静态方法：添加项目到当前项目路径的分组
    pub fn add_project_to_current_group(group_id: String, project_id: String) -> Result<bool> {
        log::info!("Adding project {} to group {}", project_id, group_id);
        let project_group_manager = Self::get_instance();
        let project_path = PreferencesManager::get_project_path();
        let path_str = project_path.to_string_lossy().to_string();

        let mut success = false;
        project_group_manager.update_project_groups(|data| {
            if let Some(pg) = data.project_groups.get_mut(&path_str) {
                if let Some(group) = pg.groups.iter_mut().find(|g| g.id == group_id) {
                    let projects = group.projects.get_or_insert_with(Vec::new);

                    // 如果项目ID尚未存在，则添加
                    if !projects.contains(&project_id) {
                        projects.push(project_id.clone());
                        info!("Project {} added to group {}", project_id, group_id);
                        success = true;
                    }
                }
            }
        })?;

        Ok(success)
    }

    // 静态方法：从当前项目路径的分组中移除项目
    pub fn remove_project_from_current_group(group_id: String, project_id: String) -> Result<()> {
        log::info!("Removing project {} from group {}", project_id, group_id);
        let project_path = PreferencesManager::get_project_path();
        let project_group_manager = Self::get_instance();
        let path_str = project_path.to_string_lossy().to_string();

        project_group_manager.update_project_groups(|data| {
            if let Some(pg) = data.project_groups.get_mut(&path_str) {
                if let Some(group) = pg.groups.iter_mut().find(|g| g.id == group_id) {
                    if let Some(projects) = &mut group.projects {
                        if projects.contains(&project_id) {
                            projects.retain(|id| id != &project_id);
                            info!("Project {} removed from group {}", project_id, group_id);
                        }
                    }
                }
            }
        })?;

        Ok(())
    }
}

// 实现Default trait为ProjectGroups
impl Default for ProjectGroups {
    fn default() -> Self {
        Self {
            groups: vec![ProjectGroupManager::default_group()],
        }
    }
}
