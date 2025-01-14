use crate::utils::constans::{DATA_FORMAT, PAGE_DIR};
use crate::utils::dirs;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub name: String,   // 页面名称
    pub remark: Option<String> , // 页面描述
    pub page_data: String,
    pub project_id: String,
    pub created_at: String,
    pub updated_at: String,
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
    pub page_data: Option<String>,
    pub project_id: String,
}

pub fn count_pages_in_project(project_id: &str) -> usize {
    let page_dir = Page::get_page_dir();
    let entries = fs::read_dir(page_dir).unwrap();
    let mut count = 0;
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let json = fs::read_to_string(&path).unwrap();
            let page: Page = serde_json::from_str(&json).unwrap();
            if page.project_id == project_id {
                count += 1;
            }
        }
    }
    count
}

impl Page {
    pub fn new(
        id: String,
        name: String,
        remark: Option<String>,
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

    pub fn get_page_dir() -> PathBuf {
        let root_dir: PathBuf = dirs::app_data_dir().unwrap();
        let page_dir: PathBuf = root_dir.join(PAGE_DIR);
        page_dir
    }

    pub fn list(
        page_num: usize,
        page_size: usize,
        project_id: Option<String>,
        keyword: Option<String>,
    ) -> Result<PageList, String> {
        let mut pages_list = vec![];
        let page_dir = Self::get_page_dir();
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir).map_err(|e| format!("创建页面目录失败: {}", e))?;
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
                if let Some(project_id) = &project_id {
                    if page.project_id != *project_id {
                        continue;
                    }
                }

                pages_list.push(page);
            }
        }
        // 分页逻辑
        let start = (page_num - 1) * page_size;
        let end = start + page_size;
        let end = end.min(pages_list.len());

        let total = pages_list.len();
        let list = pages_list[start..end].to_vec();
        Ok(PageList { total, list })
    }

    pub fn save(&self, page_file: PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(page_file, json).unwrap();
        Ok(())
    }

    pub fn load(page_file: &PathBuf) -> Page {
        println!("{}", page_file.display());
        let json = fs::read_to_string(page_file).unwrap();
        let page = serde_json::from_str(&json).unwrap();
        page
    }

    pub fn delete(id: String) -> Result<(), String> {
        let page_dir = Self::get_page_dir();
        let page_file = page_dir.join(format!("{}.json", id));
        fs::remove_file(page_file).map_err(|e| format!("删除页面失败: {}", e))?;
        Ok(())
    }

}
