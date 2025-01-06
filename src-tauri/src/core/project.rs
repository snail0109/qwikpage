use serde::{Serialize, Deserialize};
use serde_json::{self, Error};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use uuid::Uuid;
use tauri::command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: Uuid,
    name: String,
    remark: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectList {
    projects: HashMap<Uuid, Project>,
}

impl ProjectList {
    fn new() -> Self {
        ProjectList {
            projects: HashMap::new(),
        }
    }

    fn add_project(&mut self, name: String, remark: String) -> Uuid {
        let project_id = Uuid::new_v4();
        let project = Project {
            id: project_id,
            name,
            remark,
        };
        self.projects.insert(project_id, project);
        project_id
    }

    fn delete_project(&mut self, project_id: Uuid) -> bool {
        self.projects.remove(&project_id).is_some()
    }

    fn update_project(&mut self, project_id: Uuid, name: Option<String>, remark: Option<String>) -> bool {
        if let Some(project) = self.projects.get_mut(&project_id) {
            if let Some(name) = name {
                project.name = name;
            }
            if let Some(remark) = remark {
                project.remark = remark;
            }
            true
        } else {
            false
        }
    }

    fn get_project(&self, project_id: Uuid) -> Option<&Project> {
        self.projects.get(&project_id)
    }

    fn list_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(file_path).map_err(serde_json::Error::io)?;
        file.write_all(json.as_bytes()).map_err(serde_json::Error::io)?;
        Ok(())
    }

    fn load_from_file(file_path: &str) -> Result<Self, io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let project_list: ProjectList = serde_json::from_str(&contents)?;
        Ok(project_list)
    }
}

#[command]
pub fn add_project(name: String, remark: String) -> Result<Uuid, String> {
    let mut project_list = ProjectList::new();
    match ProjectList::load_from_file("project.json") {
        Ok(loaded_project_list) => {
            project_list = loaded_project_list;
        }
        Err(_) => {}
    }

    let project_id = project_list.add_project(name, remark);
    if let Err(e) = project_list.save_to_file("project.json") {
        return Err(format!("Failed to save projects to file: {}", e));
    }
    Ok(project_id)
}

#[command]
pub fn update_project(project_id: Uuid, name: Option<String>, remark: Option<String>) -> Result<bool, String> {
    let mut project_list = ProjectList::new();
    match ProjectList::load_from_file("project.json") {
        Ok(loaded_project_list) => {
            project_list = loaded_project_list;
        }
        Err(e) => return Err(format!("Failed to load projects from file: {}", e)),
    }

    let result = project_list.update_project(project_id, name, remark);
    if let Err(e) = project_list.save_to_file("project.json") {
        return Err(format!("Failed to save projects to file: {}", e));
    }
    Ok(result)
}

#[command]
pub fn delete_project(project_id: Uuid) -> Result<bool, String> {
    let mut project_list = ProjectList::new();
    match ProjectList::load_from_file("project.json") {
        Ok(loaded_project_list) => {
            project_list = loaded_project_list;
        }
        Err(e) => return Err(format!("Failed to load projects from file: {}", e)),
    }

    let result = project_list.delete_project(project_id);
    if let Err(e) = project_list.save_to_file("project.json") {
        return Err(format!("Failed to save projects to file: {}", e));
    }
    Ok(result)
}


#[derive(Serialize)]
pub struct ProjectListResult {
    list: Vec<Project>,
    total: usize
}

#[command]
pub fn get_project_list() -> Result<ProjectListResult, String> {
    // mock 一些数据返回, 返回
    let mut project_list = ProjectList::new();
    project_list.add_project("TestA".to_string(), "TestA Preoject description".to_string());
    project_list.add_project("TestB".to_string(), "TestB Preoject description".to_string());
    project_list.add_project("TestC".to_string(), "TestC Preoject description".to_string());
    project_list.add_project("TestD".to_string(), "TestD Preoject description".to_string());

    let result = ProjectListResult {
        list: project_list.projects.values().cloned().collect(),
        total: project_list.projects.len(),
    };
    Ok(result)
}
