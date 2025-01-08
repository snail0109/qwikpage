use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub remark: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectList {
    pub projects: HashMap<Uuid, Project>,
}
