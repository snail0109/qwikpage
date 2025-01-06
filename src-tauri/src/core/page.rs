use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub description: String,
    pub updated_at: String,
    pub created_at: String,
}

impl Page {

    pub fn new(id: i32, name: String, path: String, description: String, updated_at: String, created_at: String) -> Page {
        Page {
            id,
            name,
            path,
            description,
            updated_at,
            created_at,
        }
    }
}

pub struct PageList {
    pub projects: Vec<Page>,
}

impl PageList {
    pub fn new(projects: Vec<Page>) -> PageList {
        PageList { projects }
    }
}