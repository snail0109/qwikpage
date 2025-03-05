pub mod fishx;

use fishx::FishxGenerator;

use crate::code_generator::config::GeneratorConfig;
use crate::code_generator::error::Result;
use crate::models::page::{Element, Page, PageContent};


pub trait CodeGenerator {
    /// 下载模板文件
    ///
    /// # 参数
    /// * `config` - 生成器配置
    async fn download_template(&self, config: &GeneratorConfig) -> Result<()>;

    /// 导出代码
    ///
    /// # 参数
    /// * `config` - 生成器配置
    /// * `page_list` - 页面列表
    async fn export_code(&self, config: &GeneratorConfig, page_list: Vec<Page>) -> Result<()>;

    /// 导出单个页面
    ///
    /// # 参数
    /// * `index` - 页面索引
    /// * `config` - 生成器配置
    /// * `page` - 页面数据
    async fn export_page(&self, index: usize, config: &GeneratorConfig, page: &Page) -> Result<()>;

    /// 导出资源文件
    ///
    /// # 参数
    /// * `config` - 生成器配置
    async fn export_resources(&self, config: &GeneratorConfig) -> Result<()>;

     /// 生成组件代码
     fn generate_components(&self, elements: &[Element], page_data: &PageContent) -> Result<(Vec<String>, Vec<String>)>;
    
     /// 生成导入语句
     fn generate_import_statement(&self, components: &[String]) -> String;

    /// 获取生成器名称
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
pub enum Generator {
    Fishx(FishxGenerator),
    // Vue(VueGenerator),
}

impl Generator {
    pub async fn export_code(&self, config: &GeneratorConfig, page_list: Vec<Page>) -> Result<()> {
        match self {
            Generator::Fishx(g) => g.export_code(config, page_list).await,
            // Generator::Vue(g) => g.export_code(config, page_list).await,
        }
    }

    pub async fn export_page(
        &self,
        index: usize,
        config: &GeneratorConfig,
        page: &Page,
    ) -> Result<()> {
        match self {
            Generator::Fishx(g) => g.export_page(index, config, page).await,
            // Generator::Vue(g) => g.export_page(index, config, page).await,
        }
    }

    pub async fn export_resources(&self, config: &GeneratorConfig) -> Result<()> {
        match self {
            Generator::Fishx(g) => g.export_resources(config).await,
            // Generator::Vue(g) => g.export_resources(config).await,
        }
    }

    pub async fn download_template(&self, config: &GeneratorConfig) -> Result<()> {
        match self {
            Generator::Fishx(g) => g.download_template(config).await,
            // Generator::Vue(g) => g.download_template(config).await,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Generator::Fishx(g) => g.name(),
            // Generator::Vue(g) => g.name(),
        }
    }
}
