use anyhow::Result;
use code_core::types::page::Page;
use rocket::Config;
use rocket::{
    catch, catchers,
    fairing::AdHoc,
    fs::{FileServer, NamedFile},
    get,
    post,
    http::{Status, ContentType, Method},
    routes,
    serde::json::Json,
    Request, State,
};
use serde::Deserialize;
use tauri::{AppHandle, Manager};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use crate::storage::page::PageConfig;
use crate::types::project::Project;

#[derive(Deserialize)]
pub struct ProxyRequest {
    method: String,
    target_url: String,
    #[serde(default)]
    data: Option<serde_json::Value>,
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();
    
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration error")
}

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
    log::trace!("初始化项目页面预览服务");
    let resource_dir = handle
        .path()
        .resource_dir()
        .expect("获取App资源目录失败");
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
                get_page_detail_with_path,
                proxy_request,   
            ],
        )
        .attach(make_cors())
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
    log::debug!("TPreviewService::get_project_detail(): 项目ID({})", id);
    match Project::load(id) {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}


// 获取页面详情
#[get("/page/detail/id/<project_id>/<id>")]
pub fn get_page_detail(project_id:String, id: String) -> Result<Json<Page>, Status> {
    log::debug!("TPreviewService::get_page_detail(): 查询页面信息, 页面项目ID({}), 页面ID({})", project_id, id);
    match PageConfig::get_page_detail_with_id(id, project_id) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// 获取页面详情
#[get("/page/detail/<project_id>/<path>")]
pub fn get_page_detail_with_path(project_id: String, path: String) -> Result<Json<Page>, Status> {
    log::debug!("TPreviewService::get_page_detail_with_path(): 查询页面信息,页面项目ID({}), 页面路由({})", project_id, path);
    match PageConfig::get_page_detail_with_path(project_id, path) {
        Ok(page) => Ok(Json(page)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// 转发http请求
#[post("/proxy", data = "<proxy_request>")]
pub async fn proxy_request(
    proxy_request: Json<ProxyRequest>,
    content_type: Option<&ContentType>,
) -> Result<(Status, Vec<u8>), Status> {
    let method = proxy_request.method.to_uppercase();
    let target_url = &proxy_request.target_url;
    
    log::debug!("TPreviewService::proxy_request(): 转发请求到 {}", target_url);
    
    // 创建HTTP客户端
    let client = reqwest::Client::new();
    
    // 检查HTTP方法，只支持GET和POST
    if method != "GET" && method != "POST" {
        log::error!("不支持的HTTP方法: {}", method);
        return Err(Status::BadRequest);
    }
    
    // 构建并发送请求
    let response = if method == "GET" {
        client.get(target_url).send().await
    } else { // POST
        let mut request_builder = client.post(target_url);
        
        // 添加内容类型（如果提供）
        if let Some(ct) = content_type {
            request_builder = request_builder.header(reqwest::header::CONTENT_TYPE, ct.to_string());
        }
        
        // 添加请求体（如果有）
        if let Some(data) = &proxy_request.data {
            request_builder = request_builder.json(data);
        }
        
        request_builder.send().await
    };
    
    // 处理响应
    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("转发请求失败: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    
    // 获取状态码
    let status_code = response.status().as_u16();
    let rocket_status = Status::new(status_code);
    
    // 获取响应体
    let response_body = match response.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(e) => {
            log::error!("读取响应体失败: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    
    Ok((rocket_status, response_body))
}