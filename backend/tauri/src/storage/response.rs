use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

impl ErrorResponse {
    pub fn not_found(message: String) -> Self {
        Self {
            code: 404,
            message,
        }
    }
} 