mod config;
mod error;
mod generators;
mod templates;
mod utils;

use config::{ExportType, GeneratorConfig};
use error::{CodeGenError, Result};
use generators::{fishx::FishxGenerator, Generator};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tokio::fs as async_fs;

use crate::{models::page::Page, utils::get_app_root_dir};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCodeParams {
    pub project_id: String,      // 项目 ID
    pub export_type: ExportType, // 导出类型
}

pub async fn export_code(app: AppHandle, params: ExportCodeParams) -> Result<()> {
    info!("======开始导出代码========");

    info!("查询项目 {:?} 页面信息", &params.project_id);
    let page_list = Page::list_with_options(params.project_id.clone())?;

    let page_len = page_list.len();
    if page_len == 0 {
        info!("项目没有页面，导出结束");
        return Err(CodeGenError::NoPages);
    }

    // 创建代码存放目录
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code").join(&params.project_id);

    info!("创建项目代码目录: {:?}", code_dir);
    async_fs::create_dir_all(&code_dir).await?;

    // 创建生成器配置
    let template_url = params.export_type.get_template_url();
    let resource_dir = crate::utils::get_app_root_resource_dir().join(&params.project_id);

    let config = GeneratorConfig::new(
        params.project_id.clone(),
        code_dir.clone(),
        template_url,
        resource_dir,
    );

    // 根据导出类型选择对应的生成器
    let generator = match params.export_type {
        ExportType::Fishx => Generator::Fishx(FishxGenerator),
        ExportType::Fish => todo!(),
        ExportType::Vue => todo!(),
    };

    // 下载模板
    info!("开始下载模板...");
    generator.download_template(&config).await.map_err(|e| {
        error!("下载模板失败: {}", e);
        CodeGenError::DownloadError(e.to_string())
    })?;
    info!("模板下载成功");

    // 导出代码
    generator
        .export_code(&config, page_list)
        .await
        .map_err(|e| {
            error!("导出代码失败: {}", e);
            CodeGenError::ExportError(e.to_string())
        })?;

    // 导出资源
    generator.export_resources(&config).await.map_err(|e| {
        error!("导出资源文件失败: {}", e);
        CodeGenError::ExportError(e.to_string())
    })?;

    // 打开文件目录
    app.opener()
        .open_path(code_dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| {
            error!("打开文件目录失败: {}", e);
            CodeGenError::Other(e.to_string())
        })?;

    info!("======代码导出完成========");
    Ok(())
}
