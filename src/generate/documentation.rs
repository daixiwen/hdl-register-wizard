
use super::genmodel;
use std::error::Error;
use super::templates::TEMPLATES;

/*
fn format_markdown(value: &Value, output: &mut String) -> tinytemplate::error::Result<()> {
    match value {
        Value::Null => Ok(()),
        Value::Bool(b) => {
            write!(output, "{}", b)?;
            Ok(())
        }
        Value::Number(n) => {
            write!(output, "{}", n)?;
            Ok(())
        }
        Value::String(s) => {
            output.push_str(&str::replace(s,"_", r"\_"));
            Ok(())
        }
        _ => Err(tinytemplate::error::Error::GenericError{ msg: "unprintable value".to_owned()}),
    }
}
*/

pub fn generate_doc(model: &genmodel::GenModel) -> Result<String, Box<dyn Error>> {

    let markdown = TEMPLATES.render("documentation.md", &tera::Context::from_serialize(&model)?)?;
    Ok(mini_markdown::render(&markdown))
    //Ok(markdown)
}
