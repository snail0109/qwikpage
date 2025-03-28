use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Error;
use code_core::types::generator::{GeneratedArtifact, GeneratorError, GeneratorOptions};
use code_core::types::route::RouteInfo;
use handlebars::Handlebars;
use std::fs::{self, File};
use std::io::Write;

use crate::constant;

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
    ];
    Ok(for dir in dirs {
        fs::create_dir_all(output_dir.join(dir))?;
    })
}

// 初始化一些默认文件
pub fn init_files(output_dir: &Path, artifacts: &mut Vec<GeneratedArtifact>) -> Result<(), Error> {
    let temp_files = constant::template_files();
    Ok(for file in temp_files {
        let file_path = output_dir.join(&file.filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        artifacts.push(write_file(file_path, &file.content)?);
    })
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
            "vue-router": "^4.5.0"
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
            "vue-tsc": "^2.2.2"
        }
    });
    artifacts.push(write_file(
        output_dir.join("package.json"),
        &serde_json::to_string_pretty(&package_json).unwrap(),
    )?);
    Ok(())
}


// 注册自定义 helper
pub fn register_partial(handlebars: &mut Handlebars) {
    handlebars.register_template_string("views", include_str!("templates/views.hbs"))
        .unwrap();
    // 注册组件代码片段
    handlebars.register_partial("qwikpageinput", include_str!("templates/input.hbs"))
        .unwrap();
    handlebars.register_partial("qwikpagebutton", include_str!("templates/button.hbs"))
        .unwrap();
    handlebars.register_partial("qwikpageflex", include_str!("templates/flex.hbs"))
        .unwrap();
    handlebars.register_partial("qwikpagecheckbox", include_str!("templates/checkbox.hbs"))
        .unwrap();
}

// 注册自定义 helper
pub fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("style", Box::new(style_helper));
}

pub fn style_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    // 从参数中获取样式对象
    let styles: HashMap<String, String> = h
        .param(0)
        .and_then(|v| v.value().as_object())
        .map(|o| {
            o.iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
                .collect()
        })
        .unwrap();

    // 转换并拼接样式字符串
    let css = styles
        .iter()
        .map(|(k, v)| {
            let key = k
                .chars()
                .enumerate()
                .fold(String::new(), |mut acc, (i, c)| {
                    if c.is_uppercase() && i > 0 {
                        acc.push('-');
                    }
                    acc.push(c.to_ascii_lowercase());
                    acc
                });
            format!("{}: {}", key, v)
        })
        .collect::<Vec<_>>()
        .join("; ");

    out.write(&format!("{}", css))?;
    Ok(())
}
