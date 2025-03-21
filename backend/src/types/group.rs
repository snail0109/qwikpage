use serde::{Deserialize, Serialize};

use super::project::ProjectSummary;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
    pub created_at: String,
    pub updated_at: String,
    pub projects: Option<Vec<ProjectSummary>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupList {
    pub groups: Vec<GroupDetail>,
}