use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// 代码生成器配置
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// 项目ID
    pub project_id: String,
    
    /// 代码输出目录
    pub output_dir: PathBuf,
    
    /// 模板URL
    pub template_url: String,
    
    /// 资源目录
    pub resource_dir: PathBuf,
    
    /// 公共资源目录名称
    pub public_dir_name: String,
}

impl GeneratorConfig {
    /// 创建新的配置
    pub fn new(project_id: String, output_dir: PathBuf, template_url: String, resource_dir: PathBuf) -> Self {
        Self {
            project_id,
            output_dir,
            template_url,
            resource_dir,
            public_dir_name: "public".to_string(),
        }
    }
    
    /// 设置公共资源目录名称
    pub fn with_public_dir_name(mut self, name: &str) -> Self {
        self.public_dir_name = name.to_string();
        self
    }
}

/// 导出类型
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExportType {
    /// Fishx框架
    Fishx,
    
    /// Fish框架
    Fish,
    
    /// Vue框架
    Vue,
}

impl ExportType {
    /// 获取模板URL
    pub fn get_template_url(&self) -> String {
        match self {
            ExportType::Fishx => String::from("https://fish.iwhalecloud.com/qwikpage-template/fishx.zip"),
            ExportType::Vue => String::from("https://fish.iwhalecloud.com/qwikpage-template/vue.zip"),
            ExportType::Fish => String::from("https://fish.iwhalecloud.com/qwikpage-template/vue.zip"),
        }
    }

     /// 转换为字符串
     pub fn to_string(&self) -> String {
        match self {
            ExportType::Fishx => String::from("fishx"),
            ExportType::Fish => String::from("fish"),
            ExportType::Vue => String::from("vue"),
        }
    }
}
