//! handles the list of files to generate

use super::genmodel;
use serde::Deserialize;
use std::error::Error;
use tera::Tera;

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

// generates the file list for a model
pub fn generate_list(
    model: &genmodel::GenModel,
    templates: &Tera,
) -> Result<FileList, Box<dyn Error>> {
    let json_list = templates.render("list.json", &tera::Context::from_serialize(&model)?)?;

    Ok(serde_json::from_slice::<FileList>(json_list.as_bytes())?)
    //Ok(markdown)
}
