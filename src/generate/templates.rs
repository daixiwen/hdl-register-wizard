use tera::{Tera,Result};
use std::collections::HashMap;

fn escape_markdown(value : &tera::Value, _args : &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let in_string : String = tera::from_value(value.clone())?;
    let underscore_replaced = str::replace(&in_string, "_", r"\_");
    let star_replaced = str::replace(&underscore_replaced, "*", r"\*");

    Ok(tera::to_value(star_replaced)?)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_template(tera: &mut Tera, name : &str) -> Result<()> {
    let rel_fname = format!("templates/{name}");
    if let Some(template_path) = crate::assets::find_asset(&rel_fname) {
        tera.add_template_file(template_path, Some(name))?;
        Ok(())
    } else {
        Err(tera::Error::msg(format!("template file not found: {name}")))
    }
}

// the way templates are loaded depends on the target. We define a macro for this
//- for the desktop app: load the file with the load_template function
#[cfg(not(target_arch = "wasm32"))]
macro_rules! template {
    ($t: ident, $n:literal) => { load_template(&mut $t, $n)?; }
}

//- for the web app, include the template as a string in the executable
#[cfg(target_arch = "wasm32")]
macro_rules! template {
    ($t: ident, $n:literal) => { $t.add_raw_template($n, include_str!(concat!("../templates/", $n)))?; }
}

pub fn gen_templates(_settings : &crate::settings::Settings) -> Result<Tera> {
    let mut tera = Tera::default();

    tera.autoescape_on(vec![]);
    tera.register_filter("escape_markdown", escape_markdown);

    // documentation template
    template!(tera,"documentation.md");
    
    Ok(tera)
}
