use crate::FileTemplate;

pub mod view;
mod not_found;

pub use view::*;

pub fn get_views() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/views/NotFound.vue"),
        content: String::from(not_found::NOT_FOUND_TEMPLATE),
    });
    return files;
}