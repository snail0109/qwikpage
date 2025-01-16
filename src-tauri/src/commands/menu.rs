use crate::commands::page::add_page;
use crate::models::menu::{Menu, MenuParams};
use crate::utils::constans::MENU_DIR;
use crate::utils::dirs;
use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[command]
pub fn get_menu_list(
    project_id: String,
    name: Option<String>,
    status: i32,
) -> Result<Vec<Menu>, String> {
    info!("Menu::get_menu_list start, project_id: {}, name: {:?}, status: {}", project_id, name, status);
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
                    let menu: Menu =
                        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                    // 过滤
                    if let Some(ref n) = name {
                        if !menu.name.contains(n) {
                            continue;
                        }
                    }

                    if status != -1 && menu.status != status {
                        continue;
                    }
                    menu_list.push(menu);
                }
            }
        }
    }
    Ok(menu_list)
}

// menu
#[command]
pub fn add_menu(is_create: u32, params: MenuParams) -> Result<(), String> {
    info!("Menu::add_menu start, is_create: {}, params: {:?}", is_create, params);
    // 获取根目录，使用proper错误处理
    let root_dir = dirs::app_data_dir().unwrap();

    // 避免多次clone project_id
    let project_path = root_dir.join(&params.project_id);

    // 如果需要创建页面
    if is_create == 1 {
        let page_id = uuid::Uuid::new_v4().to_string();
        add_page(
            Some(page_id.clone()),
            params.name.clone(),
            Some(String::new()),
            Some(String::new()),
            params.project_id.clone(),
        )
        .map_err(|e| format!("创建页面失败: {}", e))?;

        // 更新MenuParams
        let mut create_param = params;
        create_param.page_id = Some(page_id);

        // 创建并保存菜单
        let menu_id = uuid::Uuid::new_v4().to_string();
        let menu = Menu::new(menu_id, create_param);
        menu.save(&project_path)
            .map_err(|e| format!("保存菜单失败: {}", e))?;
        info!("add_menu success");
        Ok(())
    } else {
        // 直接创建菜单
        let menu_id = uuid::Uuid::new_v4().to_string();
        let menu = Menu::new(menu_id, params);
        menu.save(&project_path)
            .map_err(|e| format!("保存菜单失败: {}", e))?;
        info!("add_menu success");
        Ok(())
    }
}

#[command]
pub fn update_menu(id: String, params: MenuParams) -> Result<(), String> {
    info!("Menu::update_menu start, id: {}, params: {:?}", id, params);
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(&params.project_id);
    let mut menu = Menu::load(&project_path , id.clone());
    menu.update(params);
    menu.save(&project_path).map_err(|e| format!("更新菜单失败: {}", e))?;
    info!("update_menu success");
    Ok(())
}

#[command]
pub fn delete_menu(id: String, project_id: String) -> Result<(), String> {
    info!("Menu::delete_menu start, id: {}, project_id: {}", id, project_id);
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    Menu::delete(&project_path, id);
    info!("delete_menu success");
    Ok(())
}

#[command]
pub fn copy_menu(id: String, project_id: String) -> Result<(), String> {
    info!("Menu::copy_menu start, id: {}, project_id: {}", id, project_id);
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    let mut menu = Menu::load(&project_path , id.clone());
    menu.copy();
    menu.save(&project_path).map_err(|e| format!("复制保存菜单失败: {}", e))?;
    info!("copy_menu success");
    Ok(())
}

#[command]
pub fn get_menu_detail(id: String, project_id: String) -> Result<Menu, String> {
    info!("Menu::get_menu_detail start, id: {}, project_id: {}", id, project_id);
    let root_dir: PathBuf = dirs::app_data_dir().unwrap();
    let project_path = root_dir.join(project_id);
    let menu = Menu::load(&project_path, id);
    info!("get_menu_detail success");
    Ok(menu)
}