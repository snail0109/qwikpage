use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    id: String,
    name: String,
    description: String,
    page_id: String,
    created_at: String,
    updated_at: String,
}
