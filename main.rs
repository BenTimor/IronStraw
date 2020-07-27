use crate::processing::full_process;
use crate::config::get_config;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::env;

mod processing;
mod config;
mod preprocessing;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // The first argument is the script itself.
    args.remove(0);

    for arg in args {
        // Getting and processing the content of the file. Adding HTML and Doctype.
        let content = full_process(
            &get_file_content(&arg.to_string()),
            &get_config(true));

        // Splitting the file type and replacing with html. If not exist, just adding html.
        // It's doing it by reversing the text and splitting the first '.'.
        let mut path = arg.chars().rev().collect::<String>();
        path = path.splitn(2, ".").nth(1).unwrap_or(&*path).to_string();
        path = path.chars().rev().collect::<String>();

        // Writing into the new file.
        export_content_into_file(&format!("{}.html", &path), &content);

        println!("{} compiled into {}.html", &arg, &path)
    }
}

/// Returns the content of a file
fn get_file_content(path: &String) -> String {
    let path_obj = Path::new(path);

    let mut file = match File::open(&path_obj) {
        Err(why) => panic!("Couldn't open the file {}. Please check again. Error: {}",  &path, why),
        Ok(file) => file
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Couldn't read the file {}. Try again or try another file. Error: {}", &path, why),
        _ => {}
    }

    content
}

/// Exports content into a file
fn export_content_into_file(path: &String, content: &String) {
    let path_obj = Path::new(&path);

    let mut file = match File::create(&path_obj) {
        Err(why) => panic!("Couldn't create the file {}. Error: {}", &path, why),
        Ok(file) => file
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Couldn't write into file {}. Error: {}", &path, why),
        _ => {}
    }
}