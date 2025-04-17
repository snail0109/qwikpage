mod utils;

use crate::{storage::page::PageConfig, types::project::Project};
use code_core::types::{
    ffi::FfiResult,
    generator::{GeneratedArtifact, GeneratorOptions},
};
use libloading::{Library, Symbol};
use log;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_opener::OpenerExt;
use tokio::fs as async_fs;
use crate::code_generator::utils::{export_resources, clear_code_dir};

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

pub async fn export_code(app: AppHandle, params: ExportCodeParams) -> Result<(), String> {
    log::info!("======开始导出代码========");

    // 获取应用资源目录
    let resource_dir = app.path().resource_dir().map_err(|e| {
        log::error!("获取应用资源目录失败: {}", e);
        return format!("获取应用资源目录失败: {}", e);
    })?;

    // 获取插件包目录
    let plugins_dir = resource_dir.join("plugins");
    log::info!("获取应用资源插件包目录: {:#?}", plugins_dir.clone());
    if !plugins_dir.exists() {
        log::error!("获取应用资源插件包目录失败, {:#?}", plugins_dir.clone());
        return Err(format!(
            "获取应用资源插件包目录失败, {:#?}",
            plugins_dir.clone()
        ));
    }

    let lib_path: PathBuf;
    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH;
        log::info!("当前是({})架构", arch);
        // Mac 只支持 x86_64 和 arm64 两种架构
        let arch_dir = if arch == "x86_64" {
            "x86_64"
        } else {
            "arm64"
        };
        lib_path = plugins_dir
            .join(arch_dir)
            .join(format!("libcode_{}.dylib", params.export_type));
    }

    #[cfg(target_os = "windows")]
    {
        lib_path = plugins_dir.join(format!("code_{}.dll", params.export_type));
    }

    // lib_path 没有值直接返回
    log::info!("获取应用资源插件: {:#?}", lib_path);
    if !lib_path.exists() {
        log::info!("lib_path, {:?}", lib_path);
        return Err(format!("插件包 {:?} 不存在，请联系应用维护人员获取插件包", lib_path));
    }

    // 加载插件包
    let lib = unsafe { Library::new(lib_path).map_err(|e| e.to_string())? };

    let window = app.get_webview_window("main").unwrap();

    log::info!("查询项目 {:?} 信息", &params.project_id);
    let project = Project::load(params.project_id.clone()).map_err(|e| {
        log::error!("查询项目 {:?} 信息失败, {:?}", &params.project_id, e);
        return format!("查询项目 {} 信息失败, {}", &params.project_id, e);
    })?;
    let page_list = PageConfig::list_with_options(params.project_id.clone())?;

    let page_len = page_list.len();
    if page_len == 0 {
        log::info!("项目没有页面，导出结束");
        return Err("项目没有页面，导出结束".to_string());
    }

    let code_export_path = project.code_export_path;
    let project_export_path = PathBuf::from(code_export_path).join(&params.project_id);

    log::info!("创建项目代码目录: {:?}", project_export_path);
    async_fs::create_dir_all(&project_export_path)
        .await
        .map_err(|e| {
            log::error!("创建代码导出根目录失败, {}", e.to_string());
            format!("创建代码导出根目录失败, {}", e.to_string())
        })?;

    // 清空目录
    if let Err(e) = clear_code_dir(&project_export_path).await {
        log::error!("{}", e);
        return Err(e.to_string());
    }

    // 按需调整
    let options = GeneratorOptions {
        project_name: project.name,
        project_variables: project.variables,
        output_dir: project_export_path.clone(),
        version: "1.0.0".into(),
        package_manager: "npm".into(),
        page_list: page_list,
    };

    unsafe {
        let generate: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> =
            lib.get(b"generate_project").map_err(|e| e.to_string())?;
        let options_json = serde_json::to_string(&options).map_err(|e| e.to_string())?;
        let options_cstring = CString::new(options_json).map_err(|e| e.to_string())?;
        let result_ptr = generate(options_cstring.as_ptr());
        let result_str = CStr::from_ptr(result_ptr)
            .to_str()
            .map_err(|e| e.to_string())?;
        let result: FfiResult<Vec<GeneratedArtifact>> =
            serde_json::from_str(result_str).map_err(|e| e.to_string())?;
        handle_generation_result(result)?;
    }

    // 导出资源
    if let Err(e) = export_resource(&params.project_id, &project_export_path).await {
        log::error!("{}", e);
        return Err(e);
    }

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
            format!("打开文件目录失败: {}", e)
        })?;

    log::info!("======代码导出完成========");
    Ok(())
}

fn handle_generation_result(result: FfiResult<Vec<GeneratedArtifact>>) -> Result<(), String> {
    if result.success {
        log::info!("Successfully generated:");
        for artifact in result.data.unwrap() {
            log::info!(
                "✓ {} ({} bytes)",
                artifact.file_path,
                artifact.content.len()
            );
        }
        log::info!("运行项目");
        log::info!("npm install && npm run dev");
    } else {
        return Err(format!("出码失败: {:#?}", result.error));
    }
    Ok(())
}

async fn export_resource(project_id: &str, code_dir: &PathBuf) -> Result<(), String> {
    // 使用基础生成器的资源导出功能
    export_resources(&project_id, &code_dir, "public").await.map_err(|e| {
        return format!("导出静态资源失败: {}", e);
    })
}