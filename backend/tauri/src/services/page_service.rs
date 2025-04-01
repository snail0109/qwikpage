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
        "TPageService::get_page_list(): 获取页面列表, 页码({}), 分页数量({}), 关键字({:?}), 项目ID({:?})",
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
        "TPageService::get_page_detail_with_id(): 获取页面详情, 页面Id({}), 项目ID({}),",
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
        "TPageService::get_page_detail_with_path(): 获取页面详情, 项目ID({}), 项目文件路径:({})",
        project_id,
        path
    );
    let page = PageConfig::get_page_detail_with_path(project_id, path)?;
    Ok(page)
}

// menu
#[command]
pub fn add_page(params: PageAddParams) -> JSResp<Page> {
    log::debug!("TPageService::add_page(): 新增页面（{:#?}", params);
    let page = PageConfig::add_page(params);
    JSResp::from(page)
}

#[command]
pub fn update_page(params: PageUpdateParams) -> JSResp<bool> {
    log::debug!("TPageService::update_page(): 更新页面({:#?})", params);
    JSResp::from(PageConfig::update(params))
}

#[command]
pub fn delete_page(id: String, project_id: String) -> JSResp<bool> {
    log::debug!("TPageService::delete_page(): 删除页面({})", id);
    JSResp::from(PageConfig::delete(id, project_id))
}

#[command]
pub fn copy_page(params: PageCopyParams) -> JSResp<String> {
    log::debug!("TPageService::copy_page(): 复制页面({:#?})", params);
    JSResp::from(PageConfig::copy(params))
}
