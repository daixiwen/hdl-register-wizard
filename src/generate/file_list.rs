//! handles the list of files to generate

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct FileEntry {
    pub template: String,
    pub path: String,
    pub filename: String,
}

#[derive(Deserialize, Clone)]
pub struct FileList {
    pub global: Vec<FileEntry>,
}
