mod utils;
mod commands;

use crate::processing::full_process;
use crate::config::get_config;
use std::env;
use crate::utils::{get_file_content, export_content_into_file};

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

