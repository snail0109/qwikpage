use serde_json::json;
use tauri::command;

use crate::models::menu::Menu;

#[command]
pub fn get_menu_list(page_id: String) -> Result<serde_json::Value, tauri::Error> {
    Ok(json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
        "homepage": null
        }
    }))
}

// menu
#[command]
pub fn add_menu(item: Menu) -> Result<(), tauri::Error> {
    Ok(())
}

#[command]
pub fn update_menu(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}

#[command]
pub fn delete_menu(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}

#[command]
pub fn copy_menu(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}
