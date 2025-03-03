use std::path::PathBuf;

use crate::models::page::Page;

pub trait CodeGenerator {

    async fn export_code (&self,  project_id : &str, code_dir: &PathBuf, page_list: Vec<Page>) -> Result<(), String>;

    async fn export_page(&self, index: usize, code_dir: PathBuf, page: &Page,  project_id : &str) -> Result<(), String>;

    async fn export_resources(&self, project_id: &str, code_dir: &PathBuf) -> Result<(), String>;

} 