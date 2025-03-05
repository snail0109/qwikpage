use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, CodeGenError>;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("下载模板失败: {0} 请关闭代理",)]
    DownloadError(String),

    #[error("导出错误: {0}")]
    ExportError(String),

    #[error("项目没有页面")]
    NoPages,

    #[error("资源错误: {0}")]
    ResourceError(String),

    #[error("IO错误: {0}")]
    Io(#[from] io::Error),

    #[error("JSON序列化错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP请求错误: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("模板处理错误: {0}")]
    TemplateError(String),

    #[error("组件生成错误: {0}")]
    ComponentError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("其他错误: {0}")]
    Other(String),
}

impl From<String> for CodeGenError {
    fn from(error: String) -> Self {
        CodeGenError::Other(error)
    }
}

impl From<&str> for CodeGenError {
    fn from(error: &str) -> Self {
        CodeGenError::Other(error.to_string())
    }
} 