use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::utils::constans::{DATA_FORMAT, MENU_DIR};

// TODO 参考 page rename_all
#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub id: String,                // 菜单id
    pub name: String,              // 菜单名称
    pub icon: Option<String>,              // 菜单图标
    pub page_id: Option<String>,   // 绑定的页面id
    pub parent_id: Option<String>, // 父级菜单id
    pub project_id: String,        // 项目id
    pub path:  Option<String>,              // 菜单路径
    pub sort_num:  Option<u32>,             // 排序
    pub status: i32,               // 状态 1-启用 2-禁用
    pub menu_type: u32,            // 类型: 1-菜单 2-按钮 3-页面
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuParams {
    pub name: String,
    pub icon: Option<String>,
    pub page_id: Option<String>,
    pub parent_id: Option<String>,
    pub project_id: String,
    pub path: Option<String>,
    pub sort_num: Option<u32>,
    pub status: i32,
    pub menu_type: u32,
    pub is_create: Option<bool>, // 是否创建页面
}


impl Menu {
    pub fn new(menu_id: String, params: MenuParams) -> Menu {
        let now = Local::now().format(DATA_FORMAT).to_string();
        Menu {
            id: menu_id,
            name: params.name,
            icon: params.icon,
            page_id: params.page_id,
            parent_id: params.parent_id,
            project_id: params.project_id,
            path: params.path,
            sort_num: params.sort_num,
            status: params.status,
            menu_type: params.menu_type,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn save(&self, project_path: &Path) -> Result<(), String> {
        let menu_path = project_path.join(MENU_DIR);
        if !menu_path.exists() {
            fs::create_dir_all(&menu_path)
            .map_err(|e| format!("创建菜单目录失败: {}", e))?;
        }
        let menu_file: std::path::PathBuf = menu_path.join(format!("{}.json", self.id));
        let menu_json = serde_json::to_string_pretty(&self)
        .map_err(|e| format!("序列化菜单数据失败: {}", e))?;
        fs::write(menu_file, menu_json)
        .map_err(|e| format!("写入菜单文件失败: {}", e))?;
    Ok(())
    }

    // TODO 替换成menu_path
    pub fn load(project_path: &Path, id: String) -> Menu {
        let menu_path = project_path.join(MENU_DIR);
        let menu_file = menu_path.join(format!("{}.json", id));
        let menu_json = fs::read_to_string(menu_file).unwrap();
        serde_json::from_str(&menu_json).unwrap()
    }

    pub fn update(&self, project_path: &Path, id: String, params: MenuParams) -> Result<(), String> {
        let mut menu = Menu::load(project_path, id.clone());
        menu.id = id;
        menu.name = params.name;
        menu.icon = params.icon;
        menu.page_id = params.page_id;
        menu.parent_id = params.parent_id;
        menu.path = params.path;
        menu.sort_num = params.sort_num;
        menu.status = params.status;
        menu.menu_type = params.menu_type;
        menu.updated_at = Local::now().format(DATA_FORMAT).to_string();
        menu.save(project_path).map_err(|e| format!("更新菜单失败: {}", e))?;
        Ok(())
    }

    pub fn delete(project_path: &Path, id: String) {
        let menu_path = project_path.join(MENU_DIR);
        let menu_file = menu_path.join(format!("{}.json", id));
        fs::remove_file(menu_file).unwrap();
    }
    #[allow(unused)]
    pub fn copy(&self, project_path: &Path, id: String) {
        let mut menu = Menu::load(project_path, id);
        menu.id = uuid::Uuid::new_v4().to_string();
        menu.created_at = Local::now().format(DATA_FORMAT).to_string();
        menu.updated_at = Local::now().format(DATA_FORMAT).to_string();
        menu.save(project_path);
    }
}
