mod constant;
mod utils;

use anyhow::Error;
use code_core::ffi::result_to_cstring;
use code_core::types::generator::{GeneratedArtifact, GeneratorError, GeneratorOptions};
use code_core::types::page::Page;
use code_core::types::route::RouteInfo;
use code_core::{pinyin_name, CodeGenerator, TemplateData};
use handlebars::{to_json, Handlebars};
use serde_json::Map;
use std::ffi::{c_char, CStr};
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use utils::{
    gen_router, generate_package_json, init_dirs, init_files, register_helpers,
    register_partial, write_file,
};


struct VueGenerator {
    page_list: Vec<Page>,
    output_dir: PathBuf,
    reg: Handlebars<'static>,
}

impl VueGenerator {
    fn new(options: &GeneratorOptions) -> Self {
        let mut reg = Handlebars::new();

        register_partial(&mut reg);

        register_helpers(&mut reg);

        Self {
            page_list: options.page_list.clone(),
            output_dir: options.output_dir.clone(),
            reg,
        }
    }
}

impl CodeGenerator for VueGenerator {
    fn init_project(&self, options: &GeneratorOptions) -> Result<Vec<GeneratedArtifact>, Error> {
        let output_dir = &options.output_dir;
        let mut artifacts = vec![];

        // 创建基础目录结构
        init_dirs(output_dir)?;

        // 初始化默认文件
        init_files(output_dir, &mut artifacts)?;

        // 生成package.json
        generate_package_json(options, output_dir, &mut artifacts)?;

        artifacts.extend(self.generate_code()?);

        Ok(artifacts)
    }

    fn generate_code(&self) -> Result<Vec<GeneratedArtifact>, Error> {
        let output_dir = PathBuf::from(self.output_dir.clone());
        let page_list = self.page_list.clone();
        let mut route_list: Vec<RouteInfo> = vec![];
        let mut artifacts: Vec<GeneratedArtifact> = vec![];

        // 遍历 page_list, 调用 generate_page
        for page in page_list {
            if !page.page_data.is_empty() {
                artifacts.push(self.generate_page(&page, &mut route_list)?);
            }
        }

        println!("route_list: {:?}", route_list.len());

        if !route_list.is_empty() {
            artifacts.push(gen_router(output_dir.clone(), route_list)?);
        }
        Ok(artifacts)
    }

    fn generate_page(
        &self,
        config: &Page,
        route_list: &mut Vec<RouteInfo>,
    ) -> Result<GeneratedArtifact, Error> {
        let output_dir = PathBuf::from(self.output_dir.clone());
        let mut name = pinyin_name(config.name.clone().as_str());
        if name.is_empty() {
            name = config.name.clone();
        }
        let component = name[..1].to_uppercase() + &name[1..];

        // 准备路由数据
        route_list.push(RouteInfo {
            path: config.path.clone(),
            name: component.clone(),
            component_path: format!("@/views/{}.vue", component),
        });

        // 处理页面数据
        let page_data = &config.page_data;

        // 处理模版数据
        let template_data = match TemplateData::from_json(page_data) {
            Ok(data) => data,
            Err(e) => {
                return Err(Error::msg(format!("Failed to parse template data: {}", e)));
            }
        };

        let mut data = Map::new();
        data.insert("components".to_string(), to_json(template_data.components));

        // 替换模版变量
        let output = match self.reg.render("views", &data) {
            Ok(output) => output,
            Err(e) => {
                return Err(Error::msg(format!("Failed to render template: {}", e)));
            }
        };
        let file_name = format!("{}.vue", component);
        let file_path = output_dir.join("src/views").join(file_name);
        write_file(file_path.clone(), &output)?;
        // 格式化文件
        format_vue_file(file_path.clone());

        Ok(GeneratedArtifact {
            file_path: file_path.to_string_lossy().to_string(),
            content: output,
        })
    }
}

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

#[no_mangle]
pub extern "C" fn generate_project(options_json: *const c_char) -> *mut c_char {
    let options = unsafe { parse_options(options_json) };
    let generator = VueGenerator::new(&options);

    let result = generator
        .init_project(&options)
        .map_err(|e| GeneratorError::Io(e.to_string()));
    unsafe { result_to_cstring(result) }
}

unsafe fn parse_options(options_json: *const c_char) -> GeneratorOptions {
    let options_str = CStr::from_ptr(options_json).to_str().unwrap();
    serde_json::from_str(options_str).unwrap()
}
