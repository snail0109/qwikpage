
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    id: String,
    name: String,
    remark: String,
    parent_id: String,
    children: Vec<String>,
}