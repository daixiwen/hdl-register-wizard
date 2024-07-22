
use super::genmodel;
use std::error::Error;
use super::templates::TEMPLATES;

pub fn generate_doc(model: &genmodel::GenModel) -> Result<String, Box<dyn Error>> {

    let markdown = TEMPLATES.render("documentation.md", &tera::Context::from_serialize(&model)?)?;
    Ok(mini_markdown::render(&markdown))
    //Ok(markdown)
}
