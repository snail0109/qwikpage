use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteInfo {
    pub path: Option<String>,
    pub name: String,
    pub component_path: String,
}
