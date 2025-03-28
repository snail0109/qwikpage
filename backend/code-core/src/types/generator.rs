use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

use super::page::Page;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorOptions {
    pub project_name: String,
    pub output_dir: PathBuf,
    pub version: String,
    pub package_manager: String, // npm/yarn/pnpm
    pub page_list: Vec<Page>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratedArtifact {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Template error: {0}")]
    Template(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
