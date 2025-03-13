use crate::error::Result;
use crate::models::page::{Element, PageContent};

use crate::code_generator::config::GeneratorConfig;
use crate::code_generator::generators::CodeGenerator;
use crate::models::page::Page;


const BASE_DIR_NAME: &str = "vue3";

const PUBLIC_RESPIRCE_PATH: &str = "vue3/public";

/// Vue 框架代码生成器
#[derive(Debug)]

pub struct VueGenerator;

impl CodeGenerator for VueGenerator {
    async fn download_template(&self, config: &GeneratorConfig) -> Result<()> {
        todo!()
    }

    async fn export_code(&self, config: &GeneratorConfig, page_list: Vec<Page>) -> Result<()> {
        todo!()
    }

    async fn export_page(&self, index: usize, config: &GeneratorConfig, page: &Page) -> Result<()> {
        todo!()
    }

    async fn export_resources(&self, config: &GeneratorConfig) -> Result<()> {
        todo!()
    }

    fn generate_components(&self, elements: &[Element], page_data: &PageContent) -> Result<(Vec<String>, Vec<String>)> {
        todo!()
    }

    fn generate_import_statement(&self, components: &[String]) -> String {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}

impl VueGenerator {

}
