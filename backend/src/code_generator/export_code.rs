use anyhow::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

use crate::{
    code_generator::core::{download_temp, export_page, handle_routes}, models::page::Page, utils::get_app_root_dir
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExportType {
    Fishx,
    Fish,
    Vue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCodeParams {
    pub project_id: String,      // 项目 ID
    pub export_type: ExportType, // 导出类型
}

pub fn export_code_main(app: AppHandle, params: ExportCodeParams) -> Result<(), Error> {
    info!("======开始导出代码========");

    info!("查询项目 {:?} 页面信息", &params.project_id);
    let page_list = Page::list_with_options(params.project_id.clone()).unwrap();

    let mut index = 1;
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
        fs::create_dir_all(&code_dir);
        // 下载模版
        match download_temp(&code_dir, params.export_type) {
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

    // 遍历页面列表并导出每个页面
    for page in &page_list {
        export_page(index, code_dir.clone(), page);
        index += 1;
    }

    // 这边处理路由文件
    handle_routes(code_dir.clone(), page_len, &page_list);

    // 打开文件目录
    app.opener()
        .open_path(code_dir.to_string_lossy().to_string(), None::<&str>);

    Ok(())
}
