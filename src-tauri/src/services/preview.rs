use crate::commands::{menu, page, project};
use crate::{models::menu::Menu, models::page::Page, models::project::Project};
use anyhow::Result;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Request, State};
use tauri::{AppHandle, Manager};

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> Option<NamedFile> {
    let handle = req.guard::<&State<AppHandle>>().await.unwrap();
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("Failed to get resource directory");
    let index_path = resource_dir.join("assets").join("admin").join("index.html");
    NamedFile::open(index_path).await.ok()
}

pub fn configure_rocket(handle: tauri::AppHandle) -> rocket::Rocket<rocket::Build> {
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("Failed to get resource directory");
    let admin_path = resource_dir.join("assets").join("admin");
    rocket::build()
        .mount(
            "/api",
            routes![
                get_project_detail,
                get_project_menus,
                get_menu_detail,
                get_page_detail
            ],
        )
        .mount("/", FileServer::from(admin_path))
        .register("/", catchers![not_found])
        .manage(handle) // 将 AppHandle 注入 Rocket 状态
}

#[get("/project/detail/<id>")]
pub fn get_project_detail(id: String) -> Result<Json<Project>, Status> {
    match project::get_project_detail(id) {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/project/menus/<id>")]
pub fn get_project_menus(id: String) -> Result<Json<Vec<Menu>>, Status> {
    match menu::get_menu_list(id, None, -1) {
        Ok(menus) => Ok(Json(menus)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/menu/detail/<project_id>/<id>")]
pub fn get_menu_detail(project_id: String, id: String) -> Result<Json<Menu>, Status> {
    match menu::get_menu_detail(id, project_id) {
        Ok(menu) => Ok(Json(menu)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/page/detail/<id>")]
pub fn get_page_detail(id: String) -> Result<Json<Page>, Status> {
    match page::get_page_detail(id) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}
