
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub description: String,
    pub updated_at: String,
    pub created_at: String,
    pub project_id: Uuid,
}