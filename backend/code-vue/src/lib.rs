mod constant;
pub mod templates;
mod utils;
mod gen_view_file;

use anyhow::Error;
use code_core::ffi::result_to_cstring;
use code_core::types::generator::{GeneratedArtifact, GeneratorError, GeneratorOptions};
use code_core::types::page::Page;
use code_core::types::route::RouteInfo;
use code_core::{pinyin_name, CodeGenerator};
use std::ffi::{c_char, CStr};
use std::path::PathBuf;
use utils::{gen_router_file, gen_proxy_config, gen_proxy_config_file, generate_app_vue_file, generate_package_json_file, init_dirs, init_files};
use gen_view_file::gen_view_file;

struct VueGenerator {
    page_list: Vec<Page>,
    output_dir: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileTemplate {
    pub filename: String,
    pub content: String,
}

impl VueGenerator {
    fn new(options: &GeneratorOptions) -> Self {
        Self {
            page_list: options.page_list.clone(),
            output_dir: options.output_dir.clone(),
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
        generate_package_json_file(options, output_dir, &mut artifacts)?;

        // 生成App.vue
        generate_app_vue_file(options, output_dir, &mut artifacts)?;

        artifacts.extend(self.generate_code()?);

        Ok(artifacts)
    }

    /**
     * 生成页面相关代码
     * 视图页面
     * 项目路由信息
     * 代理配置
     */
    fn generate_code(&self) -> Result<Vec<GeneratedArtifact>, Error> {
        let output_dir = PathBuf::from(self.output_dir.clone());
        let page_list = self.page_list.clone();
        let mut route_list: Vec<RouteInfo> = vec![];
        let mut proxy_list: Vec<String> = vec![];
        let mut artifacts: Vec<GeneratedArtifact> = vec![];

        // 遍历 page_list, 调用 generate_page
        for page in page_list {
            if !page.page_data.is_empty() {
                gen_proxy_config(&page, &mut proxy_list)?;
                artifacts.push(self.generate_page(&page, &mut route_list)?);
            }
        }

        println!("route_list: {:?}", route_list.len());

        if !route_list.is_empty() {
            artifacts.push(gen_router_file(output_dir.clone(), route_list)?);
        }
        // if !proxy_map.is_empty() {
            // println!("proxy_map: {:?}", proxy_map);
            artifacts.push(gen_proxy_config_file(output_dir.clone(), &mut proxy_list)?);
        // }
        Ok(artifacts)
    }

    /**
     * 生成页面
     * 页面名称中文转换成拼音，首字母大写
     * 页面名称确定后，生成路由配置信息
     * 
     */
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
        let file_name = format!("{}.vue", component);
        let file_result = gen_view_file(output_dir.clone(), &file_name, config)?;

        Ok(GeneratedArtifact {
            file_path: file_result.file_path,
            content: file_result.content,
        })
    }
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
