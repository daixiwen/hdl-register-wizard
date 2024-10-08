
use super::genmodel;
use std::error::Error;
use tera::Tera;

// if an error source is present, add it to the error message
fn map_tera_error(tera_error: tera::Error) -> String {
    let mut result = tera_error.to_string();

    if let Some(source_error) = tera_error.source() {
        result = format!("{result}, caused by: {source_error}");
    }

    result
}
pub fn generate_doc(model: &genmodel::GenModel, templates: &Tera) -> Result<String, Box<dyn Error>> {

//    let markdown = templates.render("documentation.md", &tera::Context::from_serialize(&model)?)?;
    let markdown = templates.render("documentation.md", &tera::Context::from_serialize(&model)?).map_err(map_tera_error)?;

    Ok(mini_markdown::render(&markdown))
    //Ok(markdown)
}
