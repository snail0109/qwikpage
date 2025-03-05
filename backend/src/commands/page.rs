use crate::commands::cmd_response::CmdResponse;
use crate::constans::PAGE_DIR;
use crate::models::page::{Page, PageAddParams, PageCopyParams, PageList, PageUpdateParams};
use crate::models::response::ErrorResponse;
use crate::utils::get_app_root_dir;
use anyhow::Result;
use log::{error, info};
use std::path::PathBuf;
use tauri::command;

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
pub fn get_page_detail_with_id(id: String, project_id: String) -> Result<Page, ErrorResponse> {
    info!("Page::get_page_detail start, id: {}", id);
    let page_dir: PathBuf = get_app_root_dir().join(project_id).join(PAGE_DIR);
    if !page_dir.exists() {
        error!("页面目录不存在");
        return Err(ErrorResponse::not_found("页面目录不存在".to_string()));
    }
    let page_file = page_dir.join(format!("{}.json", id));
    let page = Page::load(&page_file).unwrap();
    Ok(page)
}

#[command]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Page, ErrorResponse> {
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
        Page::list(1, 20, project_id, Some("".to_string())).map_err(|e| {
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

// menu
#[command]
pub fn add_page(params: PageAddParams) -> CmdResponse<Page> {
    info!("Page::add_page start, params: {:#?}", params);
    let page = Page::add_page(params);
    CmdResponse::from(page)
}

#[command]
pub fn update_page(params: PageUpdateParams) -> CmdResponse<bool> {
    info!("Page::update_page start, params: {:#?}", params);
    CmdResponse::from(Page::update(params))
}

#[command]
pub fn delete_page(id: String, project_id: String) -> CmdResponse<bool> {
    info!("Page::delete_page start, id: {}", id);
    CmdResponse::from(Page::delete(id, project_id))
}

#[command]
pub fn copy_page(params: PageCopyParams) -> CmdResponse<String> {
    info!("Page::copy_page start, params: {:#?}", params);
    CmdResponse::from(Page::copy(params))
}
