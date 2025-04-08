use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::datetime::get_current_time;

use super::project::ProjectSummary;

// 定义组结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectGroup {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub projects: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectGroups {
    pub groups: Vec<ProjectGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupWithProjectDetail {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub projects: Option<Vec<ProjectSummary>>,
    pub is_default: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupWithProjectList {
    pub groups: Vec<GroupWithProjectDetail>,
}



// 实现Default trait为ProjectGroups
impl Default for ProjectGroups {
    fn default() -> Self {
        Self {
            groups: vec![ProjectGroup::default()],
        }
    }
}

impl Default for ProjectGroup {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "默认分组".to_string(),
            is_default: true,
            projects: None,
            created_at: get_current_time(),
            updated_at: Some(get_current_time()),
        }
    }
    
}