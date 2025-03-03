use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tokio::fs as async_fs;

use crate::{
    code_generator::{
        utils::download_temp,
        fishx_generator::FishxGenerator,
        code_generator::CodeGenerator,
    },
    models::page::Page,
    utils::get_app_root_dir,
};

use super::utils::ExportType;


#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCodeParams {
    pub project_id: String,      // 项目 ID
    pub export_type: ExportType, // 导出类型
}

pub async fn export_code(app: AppHandle, params: ExportCodeParams) -> Result<(), Error> {
    info!("======开始导出代码========");

    info!("查询项目 {:?} 页面信息", &params.project_id);
    let page_list = Page::list_with_options(params.project_id.clone()).unwrap();

    let page_len = page_list.len();
    if page_len == 0 {
        info!("项目没有页面，导出结束");
        return Err(anyhow::anyhow!("项目没有页面"));
    }

    // 创建代码存放目录
    let app_data_dir = get_app_root_dir();
    let code_dir = app_data_dir.join("qwikpage-code").join(&params.project_id);
    if !code_dir.exists() {
        info!("创建项目代码目录: {:?} 成功，开始下载模版......", code_dir);
        async_fs::create_dir_all(&code_dir).await?;
        // 下载模版
        match download_temp(&code_dir, &params.export_type).await {
            Ok(_) => {
                info!("模板下载成功");
            }
            Err(e) => {
                error!("下载模板失败: {}", e);
                return Err(anyhow::anyhow!("下载模板失败: {}, 请关闭本地代理", e));
            }
        }
    } else {
        info!("项目代码目录: {:?} 已经存在，直接开始导出......", code_dir);
    }

    // 根据导出类型选择对应的生成器
    let generator = match params.export_type {
        ExportType::Fishx => FishxGenerator,
        ExportType::Fish => todo!(),                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
        ExportType::Vue => todo!(),
    };

    generator.export_code(code_dir.clone(), page_list).await.map_err(
        |e| {
            error!("导出代码失败: {}", e);
            anyhow::anyhow!("导出代码失败: {}", e)
        },
    )?;

    // 打开文件目录
    app.opener()
        .open_path(code_dir.to_string_lossy().to_string(), None::<&str>)?;
    Ok(())
}
