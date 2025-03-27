use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, CommonError>;

#[derive(Error, Debug)]
pub enum CommonError {
    // 出码
    #[error("下载模板失败: {0} 请关闭代理",)]
    DownloadError(String),

    // #[error("导出错误: {0}")]
    // ExportError(String),

    #[error("项目没有页面")]
    NoPages,

    #[error("IO错误: {0}")]
    Io(#[from] io::Error),

    #[error("JSON序列化错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP请求错误: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

impl From<String> for CommonError {
    fn from(error: String) -> Self {
        CommonError::Other(error)
    }
}

impl From<&str> for CommonError {
    fn from(error: &str) -> Self {
        CommonError::Other(error.to_string())
    }
} 