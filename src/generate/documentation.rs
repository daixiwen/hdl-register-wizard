
use super::genmodel;
use std::error::Error;
use tera::Tera;

pub fn generate_doc(model: &genmodel::GenModel, templates: &Tera) -> Result<String, Box<dyn Error>> {

    let markdown = templates.render("documentation.md", &tera::Context::from_serialize(&model)?)?;
    Ok(mini_markdown::render(&markdown))
    //Ok(markdown)
}
