use std::path::PathBuf;

use crate::models::page::Page;

pub trait CodeGenerator {

    async fn export_code (&self,  code_dir: PathBuf, page_list: Vec<Page>) -> Result<(), String>;

    async fn export_page(&self, index: usize, code_dir: PathBuf, page: &Page) -> Result<(), String>;

} 