//! Tests for the templates
//!
//! This test will just check that all templates can be run without errors on a model file that should include all possible options
use super::super::file_formats::mdf::Mdf;
use super::super::generate::{file_list, genmodel, templates, user_strings};
use super::super::settings;
use std::collections::BTreeMap;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

#[test]
fn run_all_templates() -> Result<(), Box<dyn Error>> {
    // load the example model file for the templates test
    let mut test_map_path = std::env::current_exe()?;
    test_map_path.pop();
    test_map_path.pop();
    test_map_path.pop();
    test_map_path.pop();
    test_map_path.push("src/tests");
    let mut example_file_path = test_map_path.clone();
    example_file_path.push("templates_test.regwiz");
    let example_file = File::open(example_file_path)?;
    let reader = BufReader::new(example_file);
    let model: Mdf = serde_json::from_reader(reader)?;

    // get default settings with default user strings
    let mut default_user_strings = BTreeMap::<String, String>::default();
    user_strings::load_defaults(&mut default_user_strings);
    let default_settings = settings::Settings {
        user_templates: default_user_strings,
        ..Default::default()
    };

    // create the Tera template engine
    let mut tera_engine = templates::gen_templates(&default_settings)?;
    user_strings::update_engine(&mut tera_engine, &default_settings)?;

    // change model to structure fit for generation
    let model = genmodel::GenModel::from_model(&model, &default_settings, &tera_engine)?;

    // run the list template to get a list of files to generate
    let list = file_list::generate_list(&model, &tera_engine)?;

    // generate all outputs
    let mut output_path = test_map_path.clone();
    output_path.push("output");

    let context = tera::Context::from_serialize(&model)?;
    for entry in list.global {
        let template_name = entry.template;

        let mut output_file_path = output_path.clone();
        output_file_path.push(entry.path);
        std::fs::create_dir_all(&output_file_path)?;
        output_file_path.push(entry.filename);

        println!("File entry: template {template_name}, file {output_file_path:?}");
        let output_file = File::create(output_file_path)?;

        let mut writer = BufWriter::new(output_file);

        // apply template
        let content =
            tera_engine.render(&template_name, &context)?;
        writer.write(content.as_bytes())?;
    }

    // generate interface specific outputs
    for (interface, file_list) in model.interfaces.iter().zip(list.interfaces.iter()) {
        println!("Processing interface {} ({})", interface.name, interface.interface_type_pretty);

        // interface context
        let mut context = tera::Context::from_serialize(&interface)?;
        context.insert("global", &model);

        for entry in file_list {
            let template_name = &entry.template;

            let mut output_file_path = output_path.clone();
            output_file_path.push(&entry.path);
            std::fs::create_dir_all(&output_file_path)?;
            output_file_path.push(&entry.filename);

            println!("File entry: template {template_name}, file {output_file_path:?}");
            let output_file = File::create(output_file_path)?;

            let mut writer = BufWriter::new(output_file);

            // apply template

            let content =
                tera_engine.render(&template_name, &context)?;
            writer.write(content.as_bytes())?;
        }
    }

    Ok(())
}
