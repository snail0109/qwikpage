mod constant;
mod utils;

use anyhow::Error;
use code_core::ffi::result_to_cstring;
use code_core::{
    CodeGenerator, GeneratedArtifact, GeneratorError, GeneratorOptions, MergedElement, PageConfig, PageContent, RouteInfo
};
use handlebars::{to_json, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::path::{Path, PathBuf};
use utils::{gen_router, generate_package_json, init_dirs, init_files, merge_element, register_helpers, register_partial, write_file};


// 模板数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
    pub components: Vec<MergedElement>,
}

impl TemplateData {
    pub fn from_json(json_str: &str) -> Result<Self, Error> {
        let data: PageContent = serde_json::from_str(json_str)?;
        let components = merge_element(&data.elements, &data.elements_map);
        println!("components {:?}", components);
        Ok(TemplateData { components })
    }
}

struct VueGenerator {
    pages: Vec<String>,
    page_list: Vec<PageConfig>,
    output_dir: String,
    page_route_list: Vec<RouteInfo>,
    reg: Handlebars<'static>,
}

impl VueGenerator {
    fn new(options: &GeneratorOptions) -> Self {
        let mut reg = Handlebars::new();
      
        register_partial(&mut reg);

        register_helpers(&mut reg);

        Self {
            pages: vec![],
            page_list: options.page_list.clone(),
            output_dir: options.output_dir.clone(),
            page_route_list: vec![],
            reg,
        }
    }
}

impl CodeGenerator for VueGenerator {
    fn init_project(&self, options: &GeneratorOptions) -> Result<Vec<GeneratedArtifact>, Error> {
        let output_dir = Path::new(&options.output_dir);
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
        config: &PageConfig,
        route_list: &mut Vec<RouteInfo>,
    ) -> Result<GeneratedArtifact, Error> {
        let output_dir = PathBuf::from(self.output_dir.clone());
        let name = config.name.clone();
        let component = name[..1].to_uppercase() + &name[1..];

        // 准备路由数据
        route_list.push(RouteInfo {
            path: config.path.clone(),
            name: component.clone(),
            component_path: format!("./views/{}.vue", component),
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
        write_file(output_dir.join("src/views").join(file_name), &output)
    }
}

#[no_mangle]
pub extern "C" fn generate_vue_project(options_json: *const c_char) -> *mut c_char {
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

