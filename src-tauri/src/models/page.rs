use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    id: String,
    name: String,
    content: String,
    created_at: String,
    updated_at: String,
}
