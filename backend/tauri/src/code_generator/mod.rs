mod utils;

use crate::{
    error::{CommonError, Result},
    types::{page::Page, project::Project},
};
use code_core::{FfiResult, GeneratedArtifact, GeneratorOptions};
use libloading::{Library, Symbol};
use log;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_opener::OpenerExt;
use tokio::fs as async_fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCodeParams {
    pub project_id: String,  // 项目 ID
    pub export_type: String, // 导出类型
}

#[derive(Serialize, Clone)]
struct StepPayload {
    step: String,
    message: String,
}

pub async fn export_code(app: AppHandle, params: ExportCodeParams) -> Result<()> {
    log::info!("======开始导出代码========");

    // 获取插件包目录
    let resource_dir = app
        .path()
        .resource_dir()
        .expect("Failed to get resource directory");
    let mut lib_path: PathBuf;
    #[cfg(target_os = "macos")]
    {
        lib_path = resource_dir
            .join("plugins")
            .join(format!("lib{}.dylib", params.export_type));
    }

    #[cfg(target_os = "windows")]
    {
        lib_path = resource_dir
            .join("plugins")
            .join(format!("lib{}.dylib", params.export_type));
    }

    //    lib_path 没有值直接返回
    if !lib_path.exists() {
        return Err(CommonError::Other("不支持该类型插件".to_string()));
    }

    let lib = unsafe { Library::new(lib_path).map_err(|e| CommonError::Other(e.to_string()))? };

    let window = app.get_webview_window("main").unwrap();

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

    let code_export_path = project.code_export_path;
    let project_export_path = PathBuf::from(code_export_path).join(&params.project_id);

    log::info!("创建项目代码目录: {:?}", project_export_path);
    async_fs::create_dir_all(&project_export_path).await?;

    // 按需调整
    let options = GeneratorOptions {
        project_name: project.name,
        output_dir: project_export_path.clone(),
        version: "1.0.0".into(),
        package_manager: "npm".into(),
        page_list: vec![],
    };

    unsafe {
        // 如果是mac file name 是  "lib"+params.export_type.to_string() + ".dylib", 如果是windows 则是 params.export_type.to_string() + ".dll"

        let generate: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> =
            lib.get(b"generate_project").map_err(|e| e.to_string())?;
        let options_json =
            CString::new(serde_json::to_string(&options)?).map_err(|e| e.to_string())?;
        let result_ptr = generate(options_json.as_ptr());
        let result_str = CStr::from_ptr(result_ptr)
            .to_str()
            .map_err(|e| e.to_string())?;
        let result: FfiResult<Vec<GeneratedArtifact>> = serde_json::from_str(result_str)?;
        handle_generation_result(result);
    }

    // TODO 导出资源

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

fn handle_generation_result(result: FfiResult<Vec<GeneratedArtifact>>) -> anyhow::Result<()> {
    if result.success {
        println!("Successfully generated:");
        for artifact in result.data.unwrap() {
            println!(
                "✓ {} ({} bytes)",
                artifact.file_path,
                artifact.content.len()
            );
        }
        println!("\nRun your project:");
        println!("cd dist-vue && npm install && npm run dev");
    } else {
        anyhow::bail!("Generation failed: {}", result.error.unwrap());
    }
    Ok(())
}
