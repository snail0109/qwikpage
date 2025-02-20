use crate::constans::{DATA_FORMAT, PAGE_DIR};
use crate::models::page::{Page, PageList};
use crate::models::response::ErrorResponse;
use crate::utils::get_app_root_dir;
use anyhow::Result;
use chrono::Local;
use log::{error, info};
use std::path::PathBuf;
use tauri::command;
use uuid::Uuid;

#[command]
pub fn get_page_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
    project_id: String,
) -> Result<PageList, String> {
    info!(
        "Page::get_page_list start, page_num: {}, page_size: {}, keyword: {:?}, project_id: {:?}",
        page_num, page_size, keyword, project_id
    );
    let pages_list =
        Page::list(page_num, page_size, project_id, keyword).map_err(|e| e.to_string())?;
    Ok(pages_list)
}

#[command]
pub fn get_page_detail(id: String, project_id: String) -> Result<Page, ErrorResponse> {
    info!("Page::get_page_detail start, id: {}", id);
    let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
    if !page_dir.exists() {
        error!("页面目录不存在");
        return Err(ErrorResponse::not_found("页面目录不存在".to_string()));
    }
    let page_file = page_dir.join(format!("{}.json", id));
    let page = Page::load(&page_file).map_err(|e| ErrorResponse::not_found(e))?;
    Ok(page)
}

#[command]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Page, ErrorResponse> {
    info!(
        "Page::get_page_detail_with_path start, project_id: {:?}, path: {}",
        project_id, path
    );
    let pages_list: PageList =
        Page::list(1, 20, project_id, Some("".to_string())).map_err(|e| {
            error!("Failed to list pages: {}", e);
            ErrorResponse::not_found(format!("无法获取页面列表: {}", e))
        })?;
    // 查找与给定 path 匹配的页面
    for page in pages_list.list {
        if page.path.as_ref() == Some(&format!("/{}", path)) {
            // 进行匹配
            info!("Page::getMartten, path: {}", path);
            return Ok(page);
        }
    }
    // 如果没有找到匹配的页面，返回一个错误
    Err(ErrorResponse::not_found(format!(
        "未找到匹配的页面，路径: {}",
        path
    )))
}

// menu
#[command]
pub fn add_page(
    id: Option<String>,
    name: String,
    path: Option<String>,
    remark: Option<String>,
    page_data: Option<String>,
    project_id: String,
) -> Result<(), String> {
    info!(
        "Page::add_page start, id: {:?}, name: {}, path: {:?}, remark: {:?}, project_id: {}",
        id, name, path, remark, project_id
    );
    let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
    let page_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let page = Page::new(page_id.clone(), name, path, remark, page_data);
    let page_file = page_dir.join(format!("{}.json", page_id.clone()));
    page.save(page_file)
        .map_err(|e| format!("创建页面失败: {}", e))?;
    info!("add_page success");
    Ok(())
}

#[command]
pub fn update_page(
    id: String,
    name: Option<String>,
    path: Option<String>,
    remark: Option<String>,
    page_data: Option<String>,
    project_id: String,
) -> Result<(), ErrorResponse> {
    info!(
        "Page::update_page start, id: {}, name: {:?}, path: {:?}, remark: {:?}, page_data: {:?}, project_id: {:?}",
        id, name, path, remark, page_data, project_id
    );
    let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
    let page_file = page_dir.join(format!("{}.json", id));
    let mut page = Page::load(&page_file).map_err(|e| ErrorResponse::not_found(e))?;
    if let Some(name) = name {
        page.name = name;
    }
    if let Some(path) = path {
        page.path = Some(path);
    }
    if let Some(remark) = remark {
        page.remark = Some(remark);
    }
    if let Some(page_data) = page_data {
        page.page_data = page_data;
    }
    page.updated_at = Local::now().format(DATA_FORMAT).to_string();
    page.save(page_file)
        .map_err(|e| ErrorResponse::not_found(e))?;
    info!("update_page success");
    Ok(())
}

#[command]
pub fn delete_page(id: String, project_id : String) -> Result<(), String> {
    info!("Page::delete_page start, id: {}", id);
    Page::delete(id, project_id).map_err(|e| format!("删除页面失败: {}", e))?;
    info!("delete_page success");
    Ok(())
}

#[command]
pub fn copy_page(
    id: String,
    name: String,
    path: Option<String>,
    remark: Option<String>,
    project_id: String,
) -> Result<(), String> {
    info!(
        "Page::copy_page start, id: {}, name: {}, path: {:?}, remark: {:?}, project_id: {}",
        id, name, path, remark, project_id
    );
    let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
    let page_file = page_dir.join(format!("{}.json", id));
    let source_page = Page::load(&page_file)?;
    let new_page_id = Uuid::new_v4().to_string();
    let new_page_file = page_dir.join(format!("{}.json", new_page_id));
    let page = Page::new(
        new_page_id,
        name,
        path,
        remark,
        Some(source_page.page_data),
    );
    page.save(new_page_file)
        .map_err(|e| format!("复制页面失败: {}", e))?;
    info!("copy_page success");
    Ok(())
}
