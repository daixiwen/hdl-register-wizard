// build script for hdl_register_wizard

// create HTML documentation files from markdown

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out_dir).join("live_help");
    let src_dir = Path::new("./src/live_help");

    fs::create_dir_all(dest_dir.clone()).unwrap();

    let paths = fs::read_dir(src_dir).unwrap();

    for path in paths {
        if let Ok(path) = path {

            let path = path.path();
            let markdown = fs::read_to_string(path.clone()).expect("unable to read file");
            let (_rest, parsed) = markdown_to_html::parser::parse_markdown(&markdown).expect("unable to parse markdown");

            let html =  markdown_to_html::translator::translate(parsed);

            let dest_file = dest_dir.clone().join(path.file_name().unwrap()).with_extension("html");

            fs::write(dest_file, html).expect("error writing html file");
        }
    }

    println!("cargo:rerun-if-changed=./src/live_help/*");
}
