use anyhow::Result;
use code_core::types::page::Page;
use rocket::Config;
use rocket::{
    catch, catchers,
    fairing::AdHoc,
    fs::{FileServer, NamedFile},
    get,
    http::Status,
    routes,
    serde::json::Json,
    Request, State,
};
use tauri::{AppHandle, Manager};

use crate::storage::page::PageConfig;
use crate::types::project::Project;

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> Option<NamedFile> {
    log::info!("404: {:?}", req.uri());
    let handle = req.guard::<&State<AppHandle>>().await.unwrap();
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("Failed to get resource directory");
    let index_path = resource_dir.join("assets").join("admin").join("index.html");
    NamedFile::open(index_path).await.ok()
}

#[catch(500)]
pub async fn internal_error(req: &Request<'_>) -> Option<NamedFile> {
    log::warn!("Internal server error");
    let handle = req.guard::<&State<AppHandle>>().await.unwrap();
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("Failed to get resource directory");
    let index_path = resource_dir.join("assets").join("admin").join("index.html");
    NamedFile::open(index_path).await.ok()
}

pub fn configure_rocket(handle: tauri::AppHandle) -> rocket::Rocket<rocket::Build> {
    log::info!("configure_rocket");
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("Preview::Failed to get resource directory");
    let admin_path = resource_dir.join("assets").join("admin");
    let config = Config {
        port: 8789,       // 指定端口
        ..Config::default() // 继承其他默认配置
    };
    rocket::custom(config)
        .mount(
            "/api",
            routes![
                get_project_detail,
                get_page_detail,
                get_page_detail_with_path
            ],
        )
        .mount("/", FileServer::from(admin_path))
        .register("/", catchers![not_found, internal_error])
        .manage(handle)
        .attach(AdHoc::on_shutdown("Shutdown Printer", |_| {
            Box::pin(async move {
                println!("...shutdown has commenced!");
                std::process::exit(0);
            })
        }))
}

// 获取项目详情
#[get("/project/detail/<id>")]
pub fn get_project_detail(id: String) -> Result<Json<Project>, Status> {
    log::debug!("Preview::get_project_detail: id: {}", id);
    match Project::load(id) {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}


// 获取页面详情
#[get("/page/detail/id/<project_id>/<id>")]
pub fn get_page_detail(project_id:String, id: String) -> Result<Json<Page>, Status> {
    log::debug!("Preview::get_page_detail: project_id: {}, id: {}", project_id, id);
    match PageConfig::get_page_detail_with_id(id, project_id) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// 获取页面详情
#[get("/page/detail/<project_id>/<path>")]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Json<Page>, Status> {
    log::debug!("Preview::get_page_detail_with_path: project_id: {}, path: {}", project_id, path);
    match PageConfig::get_page_detail_with_path(project_id, path) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}
