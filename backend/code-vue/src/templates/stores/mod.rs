use crate::FileTemplate;

pub mod index;
pub mod page_store;

pub fn get_store() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/stores/index.ts"),
        content: String::from(index::STORE_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/stores/pageStore.ts"),
        content: String::from(page_store::PAGE_STORE_INDEX),
    });
    return files;
}