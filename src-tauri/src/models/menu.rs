use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::utils::constans::MENU_DIR;
use crate::utils::help::get_current_time;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
            created_at: get_current_time(),
            updated_at: get_current_time(),
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

    pub fn update(&mut self, params: MenuParams) {
        self.name = params.name;
        self.icon = params.icon;
        self.page_id = params.page_id;
        self.parent_id = params.parent_id;
        self.path = params.path;
        self.sort_num = params.sort_num;
        self.status = params.status;
        self.menu_type = params.menu_type;
        self.updated_at = get_current_time();
    }

    pub fn delete(project_path: &Path, id: String) {
        let menu_path = project_path.join(MENU_DIR);
        let menu_file = menu_path.join(format!("{}.json", id));
        fs::remove_file(menu_file).unwrap();
    }
    #[allow(unused)]
    pub fn copy(&mut self) {
        self.id = uuid::Uuid::new_v4().to_string();
        self.created_at = get_current_time();
        self.updated_at = get_current_time();
    }
}
