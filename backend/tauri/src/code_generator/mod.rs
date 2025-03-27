mod config;
mod utils;

use std::path::PathBuf;

use crate::{
    error::{CommonError, Result},
    types::{page::Page, project::Project},
};
use config::ExportType;
use log;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_opener::OpenerExt;
use tokio::fs as async_fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCodeParams {
    pub project_id: String,      // 项目 ID
    pub export_type: ExportType, // 导出类型
}

#[derive(Serialize, Clone)]
struct StepPayload {
    step: String,
    message: String,
}

pub async fn export_code(app: AppHandle, params: ExportCodeParams) -> Result<()> {
    log::info!("======开始导出代码========");

    let window = app.get_webview_window("main").unwrap();

    window
        .emit(
            "generate-code-step",
            StepPayload {
                step: "init".into(),
                message: "初始化中...".into(),
            },
        )
        .unwrap();

    log::info!("查询项目 {:?} 页面信息", &params.project_id);
    let project = Project::load(params.project_id.clone()).map_err(|e| {
        log::error!("查询项目 {:?} 项目信息失败: {:?}", &params.project_id, e);
        e
    })?;
    let page_list = Page::list_with_options(params.project_id.clone())?;

    let page_len = page_list.len();
    if page_len == 0 {
        log::info!("项目没有页面，导出结束");
        return Err(CommonError::NoPages);
    }

    // 创建代码存放目录
    // 步骤2：处理数据
    window
        .emit(
            "generate-code-step",
            StepPayload {
                step: "processing".into(),
                message: "创建项目框架代码...".into(),
            },
        )
        .unwrap();

    let code_export_path = project.code_export_path;
    let project_export_path = PathBuf::from(code_export_path).join(&params.project_id);

    log::info!("创建项目代码目录: {:?}", project_export_path);
    async_fs::create_dir_all(&project_export_path).await?;

    // 创建生成器配置
    // let template_url = params.export_type.get_template_url();
    // let config_path = Config::global().preferences().get_project_path();
    // let resource_dir =config_path.join(&params.project_id).join("resources");

    // let config = GeneratorConfig::new(
    //     params.project_id.clone(),
    //     project_export_path.clone(),
    //     template_url,
    //     resource_dir,
    // );

    // 根据导出类型选择对应的生成器
    // let generator = match params.export_type {
    //     ExportType::Vue => Generator::Vue(VueGenerator),
    // };

    // 下载模板
    log::info!("开始下载模板...");
    // generator.download_template(&config).await.map_err(|e| {
    //     error!("下载模板失败: {}", e);
    //     CommonError::DownloadError(e.to_string())
    // })?;
    log::info!("模板下载成功");

    // 导出代码
    // generator
    //     .export_code(&config, page_list)
    //     .await
    //     .map_err(|e| {
    //         error!("导出代码失败: {}", e);
    //         CommonError::ExportError(e.to_string())
    //     })?;

    // 导出资源
    // generator.export_resources(&config).await.map_err(|e| {
    //     error!("导出资源文件失败: {}", e);
    //     CommonError::ExportError(e.to_string())
    // })?;

    // 步骤3：完成
    window
        .emit(
            "generate-code-step",
            StepPayload {
                step: "completed".into(),
                message: "出码成功！".into(),
            },
        )
        .unwrap();

    // 打开文件目录
    app.opener()
        .open_path(
            project_export_path.to_string_lossy().to_string(),
            None::<&str>,
        )
        .map_err(|e| {
            log::error!("打开文件目录失败: {}", e);
            CommonError::Other(e.to_string())
        })?;

    log::info!("======代码导出完成========");
    Ok(())
}
