use std::path::{Path, PathBuf};

use anyhow::Error;
use code_core::types::generator::{GeneratedArtifact, GeneratorError, GeneratorOptions};
use code_core::types::page::Page;
use code_core::types::route::RouteInfo;
use handlebars::Handlebars;
use serde_json::{self, json};
use std::fs::{self, File};
use std::io::Write;
use std::thread;

use crate::constant;
use crate::templates::{get_components, get_store, get_types, get_utils, VIEW_TEMPLATE};

// 生成路由文件
pub fn gen_router(
    output_dir: PathBuf,
    route_list: Vec<RouteInfo>,
) -> Result<GeneratedArtifact, Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("router", include_str!("templates/router.hbs"))
        .unwrap();
    let data = serde_json::json!({
        "routes": route_list
    });
    let output = handlebars.render("router", &data)?;
    write_file(output_dir.join("src/router/index.ts"), &output)
}

// 生成视图内容
pub fn gen_view(
    output_dir: PathBuf,
    file_name: &str,
    page: &Page,
) -> Result<GeneratedArtifact, Error> {
    let file_path = output_dir.join("src/views").join(file_name);
    // 创建一个自定义的 JSON 对象
    let frontend_data = json!({
        "id": page.id,
        "name": page.name,
        "path": page.path,
        "remark": page.remark,
        "pageData": page.page_data,
    });
    // 将 JSON 转换为字符串
    let page_json = serde_json::to_string_pretty(&frontend_data)
        .map_err(|e| anyhow::anyhow!("序列化失败: {}", e))?;
    let replacement = format!("const pageInfo = {};", page_json);
    let result = VIEW_TEMPLATE.replace("{{ pageInfo }}", &replacement);
    write_file(file_path.clone(), &result)?;
    format_vue_file(file_path.clone());
    Ok(GeneratedArtifact {
        file_path: file_path.to_string_lossy().to_string(),
        content: result,
    })
}

// 格式化 Vue 文件
fn format_vue_file(file_path: PathBuf) {
    // 创建一个新线程来处理格式化
    thread::spawn(move || {
        // 使用标准库的 Command
        let prettier_check = std::process::Command::new("npx")
            .args(&["--no-install", "prettier", "--version"])
            .output();

        if prettier_check.is_err() || !prettier_check.unwrap().status.success() {
            eprintln!("Warning: Prettier not available, skipping code formatting");
            return;
        }

        let output = std::process::Command::new("npx")
            .args(&["prettier", "--write", file_path.to_str().unwrap()])
            .output();

        if let Err(e) = output {
            eprintln!("Error formatting Vue file: {}", e);
        } else if !output.unwrap().status.success() {
            eprintln!("Prettier formatting failed");
        }
    });
}

// 写文件
pub fn write_file(path: impl AsRef<Path>, content: &str) -> Result<GeneratedArtifact, Error> {
    let mut file = File::create(&path)
        .map_err(|e| GeneratorError::Io(format!("Create file failed: {}", e)))?;
    file.write_all(content.as_bytes())
        .map_err(|e| GeneratorError::Io(format!("Write file failed: {}", e)))?;
    Ok(GeneratedArtifact {
        file_path: path.as_ref().to_string_lossy().into_owned(),
        content: content.into(),
    })
}

// 初始化一些静态文件夹
pub fn init_dirs(output_dir: &Path) -> Result<(), Error> {
    let dirs = [
        "public",
        "src/assets",
        "src/components",
        "src/router",
        "src/views",
        "src/stores",
        "src/types",
        "src/utils",
    ];
    Ok(for dir in dirs {
        fs::create_dir_all(output_dir.join(dir))?;
    })
}

// 初始化一些默认文件
pub fn init_files(output_dir: &Path, artifacts: &mut Vec<GeneratedArtifact>) -> Result<(), Error> {
    let mut temp_files = Vec::new();
    temp_files.extend(constant::template_files());
    temp_files.extend(get_components());
    temp_files.extend(get_store());
    temp_files.extend(get_types());
    temp_files.extend(get_utils());
    for file in temp_files {
        let file_path = output_dir.join(&file.filename);
        // let path_str = file_path.to_string_lossy();
        // if path_str.contains("src/components/") ||
        // path_str.contains("src\\components\\") {
        //     // 检查文件是否已存在
        //     if file_path.exists() {
        //         println!("文件已存在，跳过: {}", file_path.display());
        //         continue;
        //     }
        // }
        // 确保父目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        // 写入文件并记录生成的文件
        artifacts.push(write_file(file_path, &file.content)?);
    }

    Ok(())
}

// 生成 package.json 文件
pub fn generate_package_json(
    options: &GeneratorOptions,
    output_dir: &Path,
    artifacts: &mut Vec<GeneratedArtifact>,
) -> Result<(), Error> {
    let package_json = serde_json::json!({
        "name": options.project_name,
        "version": options.version,
        "scripts": {
            "dev": "vite",
            "build": "vite build"
        },
        "dependencies": {
            "ant-design-vue": "^4.2.6",
            "vue": "^3.5.13",
            "vue-router": "^4.5.0",
            "dayjs": "^1.11.10",
            "lodash-es": "^4.17.21",
            "pinia": "^3.0.1",
            "qs": "^6.12.1",
            "axios": "^0.27.2",
            "copy-to-clipboard": "^3.3.3",
            "@qwikpage/icons": "^0.0.1"
        },
        "devDependencies": {
            "@tsconfig/node22": "^22.0.0",
            "@types/node": "^22.13.4",
            "@vitejs/plugin-vue": "^5.2.1",
            "@vitejs/plugin-vue-jsx": "^4.1.1",
            "@vue/tsconfig": "^0.7.0",
            "npm-run-all2": "^7.0.2",
            "typescript": "~5.7.3",
            "vite": "^6.1.0",
            "vite-plugin-vue-devtools": "^7.7.2",
            "vue-tsc": "^2.2.2",
            "@types/lodash-es": "^4.17.12",
            "@types/qs": "^6.9.15"
        }
    });
    artifacts.push(write_file(
        output_dir.join("package.json"),
        &serde_json::to_string_pretty(&package_json).unwrap(),
    )?);
    Ok(())
}
