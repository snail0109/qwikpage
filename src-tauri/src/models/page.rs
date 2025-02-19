use crate::constans::{APP_IDENTIFIER, DATA_FORMAT, PAGE_DIR};
use crate::utils::{get_app_root_dir, paginate};
use chrono::Local;
use dirs;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde_json::Value;
use uuid::Uuid;

use crate::types::interceptor::Interceptor;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Element {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub type_name: String,
    pub name: String,
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElementConfig {
    pub id: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElementObj {
    pub config: Value,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageContent {
    pub elements: Vec<Element>,
    #[serde(rename = "elementsMap")]
    pub elements_map: HashMap<String, ElementObj>,
    pub apis: HashMap<Uuid, Value>,
    pub interceptor: Interceptor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub name: String,           // 页面名称
    pub path: Option<String>,   // 页面路由
    pub remark: Option<String>, // 页面描述
    pub page_data: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageList {
    pub list: Vec<Page>,
    pub total: usize,
}

impl Page {
    pub fn new(
        id: String,
        name: String,
        path: Option<String>,
        remark: Option<String>,
        page_data: Option<String>,
    ) -> Self {
        let now = Local::now().format(DATA_FORMAT).to_string();
        Page {
            id,
            name,
            path,
            remark,
            page_data: page_data.unwrap_or_else(|| String::new()),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn get_page_dir(project_id: &String) -> PathBuf {
        let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
        page_dir
    }

    pub fn list(
        page_num: usize,
        page_size: usize,
        project_id: String,
        keyword: Option<String>,
    ) -> Result<PageList, String> {
        let mut pages_list = vec![];
        let page_dir = Self::get_page_dir(&project_id);
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

                pages_list.push(page);
            }
        }
        // 分页逻辑
        let (list, total) = paginate(pages_list, page_num, page_size);
        Ok(PageList { total, list })
    }

    pub fn save(&self, page_file: PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(page_file, json).unwrap();
        Ok(())
    }

    pub fn load(page_file: &PathBuf) -> Result<Page, String> {
        if !page_file.exists() {
            warn!("页面文件不存在");
            return Err("页面文件不存在".to_string());
        }
        let json = fs::read_to_string(page_file).map_err(|e| format!("读取页面文件失败: {}", e))?;
        let page: Page =
            serde_json::from_str(&json).map_err(|e| format!("解析页面数据失败: {}", e))?;
        Ok(page)
    }

    pub fn delete(id: String, project_id: String) -> Result<(), String> {
        let page_dir = Self::get_page_dir(&project_id);
        let page_file = page_dir.join(format!("{}.json", id));
        fs::remove_file(page_file).map_err(|e| format!("删除页面失败: {}", e))?;
        Ok(())
    }

    // 根据页面参数查询对应页面
    pub fn list_with_options(
        project_id: String,
        path: Option<String>,
    ) -> Result<Vec<Page>, String> {
        let mut pages_list = vec![];
        let page_dir = Self::get_page_dir(&project_id);
        if !page_dir.exists() {
            warn!("页面文件不存在");
            return Err("页面文件不存在".to_string());
        }
        let entries = fs::read_dir(page_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                // 读取 json 文件内容
                if path.extension().unwrap() != "json" {
                    continue;
                }
                let json = fs::read_to_string(&path).unwrap();
                let page: Page = serde_json::from_str(&json).unwrap();
                pages_list.push(page);
            }
        }
        Ok(pages_list)
    }
}
