use tauri::command;
use crate::core::project::{Project, ProjectList};
use crate::core::page::Page;

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// page
#[command]
pub fn create_page(item: Page ) ->Result<(), tauri::Error> {
    Ok(())
}

#[command]
pub fn update_page(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}

#[command]
pub fn delete_page(page_id: String) -> Result<(), tauri::Error> {
    Ok(())
}