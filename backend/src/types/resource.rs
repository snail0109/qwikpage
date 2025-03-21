use serde::Serialize;

#[derive(Serialize)]
pub struct FontMeta {
    pub postscript_name: String,
    pub family: String,
    pub full_name: String,
}
