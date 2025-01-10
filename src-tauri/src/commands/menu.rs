use crate::commands::page::add_page;
use crate::models::menu::{Menu, MenuParams};
use crate::models::page::Page;
use crate::utils::constans::MENU_DIR;
use crate::utils::dirs;
use anyhow::Result;
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;


#[command]
pub fn get_menu_list(project_id: String, name: String, status: i32) -> Result<Vec<Menu>, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&project_id);
    // project menu 目录下的文件
    let menu_dir = project_path.join(MENU_DIR);
    let mut menu_list = vec![];
    if let Ok(entries) = fs::read_dir(menu_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let menu: Menu = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                    // 过滤
                    if !name.is_empty() && !menu.name.contains(&name) {
                        continue;
                    }
                    if status != 0 && menu.status != status {
                        continue;
                    }
                    menu_list.push(menu);
                }
            }
        }
    }
    Ok(menu_list)
}

#[command]
pub fn get_menu_detail(project_id:String, id: String) -> Result<Menu, String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&project_id);
    let menu = Menu::load(&project_path, id);
    Ok(menu)
}

// menu
#[command]
pub fn add_menu(project_id: String, is_create: i32 ,item: MenuParams) ->  Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id.clone());
    // is_create 为1时，创建页面
    let mut create_param = item;
    if is_create == 1 {
        let page_id = uuid::Uuid::new_v4().to_string();
        create_param.page_id = Some(page_id.clone());
        add_page(page_id, create_param.name.clone(), "".to_string(), "".to_string(), project_id.clone());
    }
    let menu = Menu::new(create_param);
    menu.save(&project_path);
    println!("menu created successfully");
    Ok(())
}

#[command]
pub fn update_menu(project_id: String, id: String, params: MenuParams) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    let menu = Menu::load(&project_path, id.clone());
    menu.update(&project_path, id, params);
    println!("menu updated successfully");
    Ok(())
}

#[command]
pub fn delete_menu(project_id : String, id: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    Menu::delete(&project_path, id);
    println!("menu deleted successfully");
    Ok(())
}

#[command]
pub fn copy_menu(project_id : String, id: String) -> Result<(), String> {
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    let menu = Menu::load(&project_path, id.clone());
    menu.copy(&project_path, id);
    println!("menu copied successfully");
    Ok(())
}
