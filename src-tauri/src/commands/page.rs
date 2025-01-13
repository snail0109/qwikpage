use crate::models::page::{Page, PageList, PageParams};
use crate::utils::dirs;
use anyhow::Result;
use std::fs;
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
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let pages_list;

    if let Some(project_id) = project_id {
        let project_path = root_dir.join(&project_id);
        // 查询指定项目下的页面
        pages_list = Page::list(&project_path, keyword);
    } else {
        // 查询所有项目下的页面
        pages_list = fs::read_dir(root_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let project_path = entry.path();
                if project_path.is_dir() {
                    Some(Page::list(&project_path, keyword.clone()))
                } else {
                    Some(vec![])
                }
            })
            .flatten()
            .collect();
    }

    // 分页逻辑
    let start = (page_num - 1) * page_size;
    let end = start + page_size;
    let end = end.min(pages_list.len());

    let total = pages_list.len();
    let list = pages_list[start..end].to_vec();
    Ok(PageList { total, list })
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
    id: Option<String>,
    name: String,
    remark: String,
    page_data: Option<String>,
    project_id: String,
) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id.clone());
    // 如果 id 为空，则生成一个随机 id
    let page_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let page = Page::new(page_id, name, remark, page_data, project_id);
    page.save(&project_path)
        .map_err(|e| format!("创建页面失败: {}", e))?;
    println!("创建页面成功");
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
