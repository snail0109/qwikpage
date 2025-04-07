use serde::{Deserialize, Serialize};

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