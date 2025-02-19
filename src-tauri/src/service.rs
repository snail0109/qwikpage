use crate::commands::{page, project};
use crate::models::{page::Page, project::Project};
use anyhow::Result;
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

#[catch(500)]
pub async fn internal_error(req: &Request<'_>) -> Option<NamedFile> {
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
    match project::get_project_detail(id) {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}


// 获取页面详情
#[get("/page/detail/<project_id>/<id>")]
pub fn get_page_detail(project_id:String, id: String) -> Result<Json<Page>, Status> {
    match page::get_page_detail(id, project_id) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// 获取页面详情
#[get("/page/detail/<project_id>/<path>")]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Json<Page>, Status> {
    match page::get_page_detail_with_path(project_id, path) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}
