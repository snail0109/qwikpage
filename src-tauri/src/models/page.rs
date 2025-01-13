use crate::utils::constans::{DATA_FORMAT, PAGE_DIR};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    id: String,
    name: String,   // 页面名称
    remark: String, // 页面描述
    page_data: String,
    project_id: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageList {
    pub list: Vec<Page>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PageParams {
    pub name: String,   // 页面名称
    pub remark: String, // 页面描述
    pub page_data: String,
    pub project_id: String,
}

impl Page {
    pub fn new(
        id: String,
        name: String,
        remark: String,
        page_data: Option<String>,
        project_id: String,
    ) -> Self {
        let now = Local::now().format(DATA_FORMAT).to_string();
        Page {
            id,
            name,
            remark,
            page_data: page_data.unwrap_or_else(|| String::new()),
            project_id,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn list(project_path: &Path, keyword: Option<String>) -> Vec<Self> {
        let mut pages = vec![];
        let page_dir = project_path.join(PAGE_DIR);
        if !page_dir.exists() {
            return pages;
        }
        let entries = fs::read_dir(page_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let json = fs::read_to_string(&path).unwrap();
                let page: Page = serde_json::from_str(&json).unwrap();
                if let Some(keyword) = &keyword {
                    if !page.name.contains(keyword) {
                        continue;
                    }
                }

                pages.push(page);
            }
        }
        pages
    }

    pub fn save(&self, project_path: &Path) -> Result<(), String> {
        let page_dir = project_path.join(PAGE_DIR);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir).map_err(|e| format!("创建页面目录失败: {}", e))?;
        }
        let page_file: std::path::PathBuf = page_dir.join(format!("{}.json", self.id));
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(page_file, json).unwrap();
        Ok(())
    }

    pub fn load(project_path: &Path, id: String) -> Page {
        let page_path = project_path.join(PAGE_DIR);
        let page_file = page_path.join(format!("{}.json", id));
        let json = fs::read_to_string(page_file).unwrap();
        let page = serde_json::from_str(&json).unwrap();
        page
    }

    pub fn update(&self, project_path: &Path, id: String, params: PageParams) {
        let mut page = Page::load(project_path, id.clone());
        page.id = id;
        page.name = params.name;
        page.remark = params.remark;
        page.page_data = params.page_data;
        page.updated_at = Local::now().format(DATA_FORMAT).to_string();
        page.save(project_path);
    }

    pub fn delete(project_path: &Path, id: String) {
        let page_file = project_path.join(PAGE_DIR).join(&id);
        fs::remove_file(page_file).unwrap();
    }

    pub fn copy(&self, project_path: &Path, id: String) {
        let mut page = Page::load(project_path, id);
        page.id = uuid::Uuid::new_v4().to_string();
        page.created_at = Local::now().format(DATA_FORMAT).to_string();
        page.updated_at = Local::now().format(DATA_FORMAT).to_string();
        page.save(project_path);
    }
}
