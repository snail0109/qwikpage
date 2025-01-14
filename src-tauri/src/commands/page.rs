use crate::models::page::{Page, PageList};
use crate::utils::constans::{DATA_FORMAT, PAGE_DIR};
use crate::utils::dirs;
use anyhow::Result;
use chrono::Local;
use std::path::PathBuf;
use tauri::command;
use uuid::Uuid;

#[command]
pub fn get_page_list(
    page_num: usize,
    page_size: usize,
    keyword: Option<String>,
    project_id: Option<String>,
) -> Result<PageList, String> {
    let pages_list =
        Page::list(page_num, page_size, project_id, keyword).map_err(|e| e.to_string())?;
    Ok(pages_list)
}

#[command]
pub fn get_page_detail(id: String) -> Result<Page, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let page_dir: PathBuf = root_dir.join(PAGE_DIR);
    let page_file = page_dir.join(format!("{}.json", id));
    let page = Page::load(&page_file);
    Ok(page)
}

// menu
#[command]
pub fn add_page(
    id: Option<String>,
    name: String,
    remark: Option<String>,
    page_data: Option<String>,
    project_id: String,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let page_dir: PathBuf = root_dir.join(PAGE_DIR);
    let page_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let page = Page::new(page_id.clone(), name, remark, page_data, project_id);
    let page_file = page_dir.join(format!("{}.json", page_id.clone()));
    page.save(page_file)
        .map_err(|e| format!("创建页面失败: {}", e))?;
    println!("创建页面成功");
    Ok(())
}

#[command]
pub fn update_page(
    id: String,
    name: Option<String>,
    remark: Option<String>,
    page_data: Option<String>,
    project_id: Option<String>,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let page_dir: PathBuf = root_dir.join(PAGE_DIR);
    let page_file = page_dir.join(format!("{}.json", id));
    let mut page = Page::load(&page_file);
    if let Some(project_id) = project_id {
        page.project_id = project_id;
    }
    if let Some(name) = name{
        page.name = name;
    }
    if let Some(remark) = remark {
        page.remark = Some(remark);
    }
    if let Some(page_data) = page_data {
        page.page_data = page_data;
    }
    page.updated_at = Local::now().format(DATA_FORMAT).to_string();
    page.save(page_file)
        .map_err(|e| format!("更新页面失败: {}", e))?;
    println!("page updated successfully");
    Ok(())
}

#[command]
pub fn delete_page(id: String) -> Result<(), String> {
    Page::delete(id).map_err(|e| format!("删除页面失败: {}", e))?;
    println!("page deleted successfully");
    Ok(())
}

#[command]
pub fn copy_page(
    id: String,
    name: String,
    remark: Option<String>,
    project_id: String,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let page_dir: PathBuf = root_dir.join(PAGE_DIR);
    let page_file = page_dir.join(format!("{}.json", id));
    let source_page = Page::load(&page_file);
    let new_page_id = Uuid::new_v4().to_string();
    let new_page_file = page_dir.join(format!("{}.json", new_page_id));
    let page = Page::new(
        new_page_id,
        name,
        remark,
        Some(source_page.page_data),
        project_id,
    );
    page.save(new_page_file)
        .map_err(|e| format!("复制页面失败: {}", e))?;
    println!("复制页面成功");
    Ok(())
}
