use crate::manager::preference_manager::PreferencesManager;
use crate::storage::response::ErrorResponse;
use crate::types::page::{PageAddParams, PageCopyParams, PageList, PageUpdateParams};
use crate::utils::datetime::get_current_time;
use crate::utils::file::is_valid_file;
use crate::utils::paginate;
use anyhow;
use code_core::types::page::Page;
use log;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use uuid::Uuid;

pub struct PageConfig {}

impl PageConfig {
    pub fn new(
        id: String,
        name: String,
        path: Option<String>,
        remark: Option<String>,
        page_data: Option<String>,
        project_id: String,
    ) -> Page {
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
        let root_dir = &PreferencesManager::get_project_path();
        let page_dir: PathBuf = root_dir.join(project_id).join("pages");
        page_dir
    }

    pub fn list(
        page_num: usize,
        page_size: usize,
        project_id: String,
        keyword: Option<String>,
    ) -> Result<PageList, String> {
        log::info!(
            "获取页面列表: page_num={}, page_size={}, project_id={}",
            page_num,
            page_size,
            project_id
        );
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

    pub fn save(page_content: Page, page_file: PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&page_content).unwrap();
        fs::write(page_file, json).unwrap();
        Ok(())
    }

    pub fn load(page_file: &PathBuf) -> io::Result<Page> {
        log::info!("加载页面文件: {}", page_file.display());
        if !page_file.exists() {
            log::warn!("页面文件不存在");
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
        log::info!("删除页面: {:#?}", page_file.clone());
        fs::remove_file(page_file).map_err(|e| format!("删除页面失败: {}", e))?;
        Ok(true)
    }

    // 根据页面参数查询对应页面
    pub fn list_with_options(project_id: String) -> Result<Vec<Page>, String> {
        let mut pages_list = vec![];
        let page_dir = Self::get_page_dir(&project_id);
        if !page_dir.exists() {
            log::warn!("页面文件不存在");
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

    // 检查页面名称和路径是否重复
    fn check_duplicate(project_id: &str, name: &str, path: Option<&String>, exclude_id: Option<&str>) -> Result<(), String> {
        let pages = Self::list_with_options(project_id.to_string())?;
        for page in pages {
            // 如果是更新操作，跳过当前页面
            if let Some(exclude_id) = exclude_id {
                if page.id == exclude_id {
                    continue;
                }
            }
            
            // 检查名称是否重复
            if page.name == name {
                return Err(format!("页面名称 '{}' 已存在", name));
            }
            
            // 检查路径是否重复
            if let Some(path) = path {
                if let Some(page_path) = page.path {
                    if page_path == *path {
                        return Err(format!("页面路径 '{}' 已存在", path));
                    }
                }
            }
        }
        Ok(())
    }

    // 新增页面
    pub fn add_page(params: PageAddParams) -> Result<Page, String> {
        // 检查重复
        Self::check_duplicate(&params.project_id, &params.name, params.path.as_ref(), None)?;

        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        let page_id = Uuid::new_v4().to_string();
        let page = PageConfig::new(
            page_id.clone(),
            params.name,
            params.path,
            params.remark,
            params.page_data,
            params.project_id,
        );
        let page_file = page_dir.join(format!("{}.json", page_id.clone()));
        PageConfig::save(page.clone(), page_file)
            .map_err(|e| format!("保存页面失败: {}", e))?;
        Ok(page)
    }

    // 更新页面
    pub fn update(params: PageUpdateParams) -> Result<bool, anyhow::Error> {
        // 检查重复
        if let Some(name) = &params.name {
            Self::check_duplicate(&params.project_id, name, params.path.as_ref(), Some(&params.id))
                .map_err(|e| anyhow::anyhow!(e))?;
        }

        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir)?;
        }
        let page_file = page_dir.join(format!("{}.json", params.id));
        let mut page = PageConfig::load(&page_file)?;
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
        PageConfig::save(page, page_file)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(true)
    }

    pub fn copy(params: PageCopyParams) -> Result<String, anyhow::Error> {
        let page_dir = Self::get_page_dir(&params.project_id);
        if !page_dir.exists() {
            fs::create_dir_all(&page_dir)?;
        }
        let page_file = page_dir.join(format!("{}.json", params.id));
        let source_page = PageConfig::load(&page_file).unwrap();
        let new_page_id = Uuid::new_v4().to_string();
        let new_page_file = page_dir.join(format!("{}.json", &new_page_id));
        log::info!("复制生成的新文件路径: {}", new_page_file.to_str().unwrap());
        let page = PageConfig::new(
            new_page_id.clone(),
            params.name,
            params.path,
            params.remark,
            Some(source_page.page_data),
            params.project_id.clone(),
        );
        PageConfig::save(page, new_page_file)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(new_page_id)
    }

    pub fn get_page_detail_with_id(id: String, project_id: String) -> Result<Page, ErrorResponse> {
        let page_dir = Self::get_page_dir(&project_id);
        if !page_dir.exists() {
            log::error!("页面目录不存在");
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
        log::info!(
            "根据项目ID和页面路由查询页面信息，项目ID({:?}),页面路由({})",
            project_id,
            path
        );
        // 如果 path 是 "*", 则将其处理为 "/"
        let effective_path = if path == "*" {
            "/".to_string()
        } else {
            format!("/{}", path)
        };
        let pages_list: PageList =
            Self::list(1, 20, project_id, Some("".to_string())).map_err(|e| {
                log::error!("无法获取页面列表: {}", e);
                ErrorResponse::not_found(format!("无法获取页面列表: {}", e))
            })?;
        // 查找与给定 path 匹配的页面
        for page in pages_list.list {
            if page.path.as_ref() == Some(&effective_path) {
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
