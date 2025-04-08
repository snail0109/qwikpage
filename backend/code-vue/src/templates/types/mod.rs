use crate::FileTemplate;

pub mod types_index;

pub fn get_types() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/types/index.ts"),
        content: String::from(types_index::TYPES_INDEX),
    });
    return files;
}