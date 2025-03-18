use crate::services::cmd_response::CmdResponse;
use crate::models::page::{Page, PageAddParams, PageCopyParams, PageList, PageUpdateParams};
use crate::models::response::ErrorResponse;
use anyhow::Result;
use log::info;
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
    let page = Page::get_page_detail_with_id(id, project_id)?;
    Ok(page)
}

#[command]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Page, ErrorResponse> {
    let page = Page::get_page_detail_with_path(project_id, path)?;
    Ok(page)
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
