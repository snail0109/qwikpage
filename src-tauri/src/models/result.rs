use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: i32,        // 状态码 (例如: 200, 400, 500)
    pub message: String,    // 响应消息
    pub data: Option<T>,    // 可选数据部分，T 是泛型，表示数据的类型
}

impl<T> ApiResponse<T> {
    // 创建成功的响应
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: 200,
            message: "OK".to_string(),
            data: Some(data),
        }
    }

    // 创建失败的响应
    pub fn error(message: String, status: i32) -> Self {
        ApiResponse {
            status,
            message,
            data: None,
        }
    }
}
