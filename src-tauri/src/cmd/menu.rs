use tauri::command;

use crate::types::menu::Menu; 

#[command]
pub fn get_menu_list(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}
// menu
#[command]
pub fn add_menu(item: Menu ) ->Result<(), tauri::Error> {
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