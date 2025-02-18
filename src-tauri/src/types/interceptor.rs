
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interceptor {
    pub headers: Vec<Header>,
    pub timeout: u32,
    #[serde(rename = "timeoutErrorMessage")]
    pub timeout_error_message: String,
}
