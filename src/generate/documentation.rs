
use super::genmodel;
use std::error::Error;
use serde_json::Value;
use std::fmt::Write;

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

pub fn generate_doc(model: &genmodel::GenModel) -> Result<String, Box<dyn Error>> {

    let mut tt = tinytemplate::TinyTemplate::new();
    tt.set_default_formatter(&format_markdown);

    tt.add_template("documentation", include_str!("templates/documentation.md"))?;
    tt.add_template("documentation_interface", include_str!("templates/documentation_interface.md"))?;

    Ok(mini_markdown::render(&tt.render("documentation", model)?))
//    Ok(tt.render("documentation", model)?)
}
