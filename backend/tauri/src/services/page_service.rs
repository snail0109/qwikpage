use crate::storage::page::PageConfig;
use crate::storage::response::ErrorResponse;
use crate::types::js_resp::JSResp;
use crate::types::page::{PageAddParams, PageCopyParams, PageList, PageUpdateParams};
use anyhow::Result;
use code_core::types::page::Page;
use tauri::command;

// FIXME:  JSResp
#[command]
pub fn get_page_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
    project_id: String,
) -> Result<PageList, String> {
    log::debug!(
        "Page::get_page_list start, page_num: {}, page_size: {}, keyword: {:?}, project_id: {:?}",
        page_num, page_size, keyword, project_id
    );
    let pages_list =
    PageConfig::list(page_num, page_size, project_id, keyword).map_err(|e| e.to_string())?;
    Ok(pages_list)
}

// FIXME:  JSResp
#[command]
pub fn get_page_detail_with_id(id: String, project_id: String) -> Result<Page, ErrorResponse> {
    log::debug!(
        "Page::get_page_detail_with_id start, id: {}, project_id: {}",
        id,
        project_id
    );
    let page = PageConfig::get_page_detail_with_id(id, project_id)?;
    Ok(page)
}

// FIXME:  JSResp
#[command]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Page, ErrorResponse> {
    log::debug!(
        "Page::get_page_detail_with_path start, project_id: {}, path: {}",
        project_id,
        path
    );
    let page = PageConfig::get_page_detail_with_path(project_id, path)?;
    Ok(page)
}

// menu
#[command]
pub fn add_page(params: PageAddParams) -> JSResp<Page> {
    log::debug!("Page::add_page start, params: {:#?}", params);
    let page = PageConfig::add_page(params);
    JSResp::from(page)
}

#[command]
pub fn update_page(params: PageUpdateParams) -> JSResp<bool> {
    log::debug!("Page::update_page start, params: {:#?}", params);
    JSResp::from(PageConfig::update(params))
}

#[command]
pub fn delete_page(id: String, project_id: String) -> JSResp<bool> {
    log::debug!("Page::delete_page start, id: {}", id);
    JSResp::from(PageConfig::delete(id, project_id))
}

#[command]
pub fn copy_page(params: PageCopyParams) -> JSResp<String> {
    log::debug!("Page::copy_page start, params: {:#?}", params);
    JSResp::from(PageConfig::copy(params))
}
