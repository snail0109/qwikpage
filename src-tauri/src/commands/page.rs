use crate::models::page::{Page, PageParams};
use crate::utils::dirs;
use anyhow::Result;
use std::path::PathBuf;
use tauri::command;
use uuid::Uuid;

#[command]
pub fn get_page_list(project_id: String, name: String) -> Result<Vec<Page>, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&project_id);
    // project pages 目录下的文件
    let pages = Page::list(&project_path, name);
    Ok(pages)
}

#[command]
pub fn get_page_detail(project_id: String, id: String) -> Result<Page, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&project_id);
    let page = Page::load(&project_path, id);
    Ok(page)
}

// menu
#[command]
pub fn add_page(
    id: String,
    name: String,
    remark: String,
    page_data: String,
    project_id: String,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id.clone());
    // 如果 id 为空，则生成一个随机 id
    let mut page_id = id;
    if page_id.is_empty() {
        page_id = Uuid::new_v4().to_string();
    }
    let page = Page::new(page_id, name, remark, page_data, project_id);
    page.save(&project_path);
    println!("menu created successfully");
    Ok(())
}

#[command]
pub fn update_page(
    id: String,
    name: String,
    remark: String,
    page_data: String,
    project_id: String,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id.clone());
    let params = PageParams {
        name,
        remark,
        page_data,
        project_id,
    };
    let page = Page::load(&project_path, id.clone());
    page.update(&project_path, id, params);
    println!("page updated successfully");
    Ok(())
}

#[command]
pub fn delete_page(project_id: String, id: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    Page::delete(&project_path, id);
    println!("page deleted successfully");
    Ok(())
}

#[command]
pub fn copy_page(project_id: String, id: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    let page = Page::load(&project_path, id.clone());
    let page_id = Uuid::new_v4().to_string();
    page.copy(&project_path, page_id);
    println!("page copied successfully");
    Ok(())
}
