use serde::{Serialize, Deserialize};

use super::generator::GeneratorError;

// FFI兼容的类型转换
#[derive(Serialize, Deserialize)]
pub struct FfiResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> From<Result<T, GeneratorError>> for FfiResult<T> {
    fn from(result: Result<T, GeneratorError>) -> Self {
        match result {
            Ok(data) => FfiResult {
                success: true,
                data: Some(data),
                error: None,
            },
            Err(e) => FfiResult {
                success: false,
                data: None,
                error: Some(e.to_string()),
            },
        }
    }
}
