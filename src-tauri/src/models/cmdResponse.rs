use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdResponse<T> {
    pub status: u32,        // 状态码 (例如: 200, 400, 500)
    pub message: Option<String>,    // 响应消息
    pub data: Option<T>,    // 可选数据部分，T 是泛型，表示数据的类型
}

impl<T> CmdResponse<T> {
    // 创建成功的响应
    pub fn success(data: T) -> Self {
        CmdResponse {
            status: 200,
            message: None,
            data: Some(data),
        }
    }

    // 创建失败的响应
    pub fn error(message: String, status: u32) -> Self {
        CmdResponse {
            status,
            message: Some(message),
            data: None,
        }
    }
}
