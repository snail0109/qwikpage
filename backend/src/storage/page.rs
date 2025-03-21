use crate::storage::response::ErrorResponse;
use crate::types::page::{Page, PageAddParams, PageCopyParams, PageList, PageUpdateParams};
use crate::utils::datetime::get_current_time;
use crate::utils::file::is_valid_file;
use crate::utils::paginate;
use anyhow::Error;
use log::{error, info, warn};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use uuid::Uuid;

use super::config::Config;

impl Page {
    pub fn new(
        id: String,
        name: String,
        path: Option<String>,
        remark: Option<String>,
        page_data: Option<String>,
        project_id: String,
    ) -> Self {
        Page {
            id,
            name,
            path,
            remark,
            page_data: page_data.unwrap_or_else(|| String::new()),
            created_at: get_current_time(),
            updated_at: get_current_time(),
            project_id,
        }
    }

    pub fn get_page_dir(project_id: &String) -> PathBuf {
        let root_dir = &Config::global().preferences().get_project_path();
        let page_dir: PathBuf = root_dir.join(project_id).join("pages");
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
            if is_valid_file(&path) {
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

    pub fn load(page_file: &PathBuf) -> io::Result<Self> {
        if !page_file.exists() {
            warn!("页面文件不存在");
            return Err(io::Error::new(ErrorKind::NotFound, "页面文件不存在"));
        }
        match fs::read_to_string(page_file) {
            Ok(data) => {
                let page: Page = serde_json::from_str(&data).unwrap();
                Ok(page)
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: String, project_id: String) -> Result<bool, String> {
        let page_dir = Self::get_page_dir(&project_id);
        let page_file = page_dir.join(format!("{}.json", id));
        fs::remove_file(page_file).map_err(|e| format!("删除页面失败: {}", e))?;
        Ok(true)
    }

    // 根据页面参数查询对应页面
    pub fn list_with_options(project_id: String) -> Result<Vec<Page>, String> {
        let mut pages_list = vec![];
        let page_dir = Self::get_page_dir(&project_id);
        if !page_dir.exists() {
            warn!("页面文件不存在");
            return Ok(pages_list);
        }
        let entries = fs::read_dir(page_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if is_valid_file(&path) {
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

    // 新增页面
    pub fn add_page(params: PageAddParams) -> Result<Page, String> {
        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        let page_id = Uuid::new_v4().to_string();
        let page = Page::new(
            page_id.clone(),
            params.name,
            params.path,
            params.remark,
            params.page_data,
            params.project_id,
        );
        let page_file = page_dir.join(format!("{}.json", page_id.clone()));
        page.save(page_file)
            .map_err(|e| format!("保存页面失败: {}", e))?;
        Ok(page)
    }

    // 更新页面
    pub fn update(params: PageUpdateParams) -> Result<bool, Error> {
        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir)?;
        }
        let page_file = page_dir.join(format!("{}.json", params.id));
        let mut page = Page::load(&page_file)?;
        if let Some(name) = params.name {
            page.name = name;
        }
        if let Some(path) = params.path {
            page.path = Some(path);
        }
        if let Some(remark) = params.remark {
            page.remark = Some(remark);
        }
        if let Some(page_data) = params.page_data {
            page.page_data = page_data;
        }
        page.updated_at = get_current_time();
        page.save(page_file)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(true)
    }

    pub fn copy(params: PageCopyParams) -> Result<String, Error> {
        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir)?;
        }
        let page_file = page_dir.join(format!("{}.json", params.id));
        let source_page = Page::load(&page_file).unwrap();
        let new_page_id = Uuid::new_v4().to_string();
        let new_page_file = page_dir.join(format!("{}.json", &new_page_id));
        info!("new_page_file: {}", new_page_file.to_str().unwrap());
        let page = Page::new(
            new_page_id.clone(),
            params.name,
            params.path,
            params.remark,
            Some(source_page.page_data),
            params.project_id.clone(),
        );
        page.save(new_page_file)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(new_page_id)
    }

    pub fn get_page_detail_with_id(id: String, project_id: String) -> Result<Page, ErrorResponse> {
        info!("Page::get_page_detail start, id: {}", id);
        let page_dir = Self::get_page_dir(&project_id);
        if !page_dir.exists() {
            error!("页面目录不存在");
            return Err(ErrorResponse::not_found("页面目录不存在".to_string()));
        }
        let page_file = page_dir.join(format!("{}.json", id));
        let page = Self::load(&page_file).unwrap();
        Ok(page)
    }

    pub fn get_page_detail_with_path(
        project_id: String,
        path: String,
    ) -> Result<Page, ErrorResponse> {
        info!(
            "Page::get_page_detail_with_path start, project_id: {:?}, path: {}",
            project_id, path
        );
        // 如果 path 是 "*", 则将其处理为 "/"
        let effective_path = if path == "*" {
            "/".to_string()
        } else {
            format!("/{}", path)
        };
        let pages_list: PageList =
            Self::list(1, 20, project_id, Some("".to_string())).map_err(|e| {
                error!("Failed to list pages: {}", e);
                ErrorResponse::not_found(format!("无法获取页面列表: {}", e))
            })?;
        // 查找与给定 path 匹配的页面
        for page in pages_list.list {
            if page.path.as_ref() == Some(&effective_path) {
                // 进行匹配
                info!("Page::getMartten, path: {}", effective_path);
                return Ok(page);
            }
        }
        // 如果没有找到匹配的页面，返回一个错误
        Err(ErrorResponse::not_found(format!(
            "未找到匹配的页面，路径: {}",
            effective_path
        )))
    }
}
