use crate::commands::{project, menu, page};
use crate::{
    models::project::Project, 
    models::menu::Menu,
    models::page::Page,
};
use anyhow::Result;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/<id>")]
pub fn get_project_detail(id: String) -> Result<Json<Project>, Status> {
    match project::get_project_detail(id) {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/<id>")]
pub fn get_project_menus(id: String) -> Result<Json<Vec<Menu>>, Status> {
    match menu::get_menu_list(id, None, -1) {
        Ok(menus) => Ok(Json(menus)),
        Err(_) => Err(Status::InternalServerError),
    }
}



#[get("/<project_id>/<id>")]
pub fn get_menu_detail(project_id:String, id: String) -> Result<Json<Menu>, Status> {
    match menu::get_menu_detail(id, project_id) {
        Ok(menu) => Ok(Json(menu)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/<id>")]
pub fn get_page_detail(id: String) -> Result<Json<Page>, Status> {
    match page::get_page_detail(id) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}


