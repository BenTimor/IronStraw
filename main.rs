mod utils;
mod commands;

use crate::processing::full_process;
use crate::config::{Config, get_config};
use std::env;
use crate::utils::{get_file_content, export_content_into_file, create_directory_if_not_exist, get_argument_parameter, has_argument_parameter, debug};
use std::fs::{metadata, read_dir};

mod processing;
mod config;
mod preprocessing;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // The first argument is the script itself.
    args.remove(0);

    /* Here comes the main arguments */

    // --debug option
    let mut to_debug = false;
    if has_argument_parameter(&"--debug".to_string(), &mut args) {
        to_debug = true;
    }

    // --target Option
    let mut directory= "./".to_string();
    // If the argument of the target exist, change the target directory
    if let Option::Some(arg) = get_argument_parameter(&"--target".to_string(), &mut args) {
        directory = arg.clone();
        create_directory_if_not_exist(&directory);
        if !directory.ends_with("/") {
            directory = format!("{}/", &directory);
        }
    }

    // --XML Option
    loop {
        if let Option::Some(arg) = get_argument_parameter(&"--XML".to_string(), &mut args) {
            compile(&arg, &directory, &get_config(false, to_debug));
        } else {
            break;
        }
    }

    /* End main arguments */

    // Call to the compile function for each argument
    for arg in args {
        compile(&arg, &directory, &get_config(true, to_debug));
    }
}

/// Recursively compiling all files in a directory
fn compile(path: &String, directory: &String, config: &Config) {
    // Getting the metadata of the file and checking if it's a real path
    let md = metadata(&path);
    if md.is_err() {
        println!("The file/directory {} doesn't seem to exist.", &path);
        return;
    }

    let unwrapped_md = md.unwrap();
    // If it's a dir, take its content and compile recursively
    if unwrapped_md.is_dir() {
        debug("main.rs::compile entering into a directory.".to_string(), &config);
        let full_directory = format!("{}{}", &directory, &path);
        create_directory_if_not_exist(&full_directory);
        let paths = read_dir(&path).unwrap();
        for path in paths {
            compile(&path.unwrap().path().display().to_string(), &directory, &config);
        }
    } else { // If it's a file, compile the file
        compile_file(&path, &directory, &config);
    }
}

/// Compiling a specific file
fn compile_file(path: &String, directory: &String, config: &Config) {
    // If it's not .sw file, skip
    if !path.ends_with(".sw") {
        debug(format!("main.rs::compile_file skipping the file {}.", &path), &config);
        return;
    }

    println!("\nStarting to render {}", &path);
    // Getting and processing the content of the file. Adding HTML and Doctype.
    let content = full_process(
        &get_file_content(&path.to_string()),
        &config);

    // Splitting the file type and replacing with html. If not exist, just adding html.
    // It's doing it by reversing the text and splitting the first '.'.
    let mut modified_path = path.chars().rev().collect::<String>();
    modified_path = modified_path.splitn(2, ".").nth(1).unwrap().to_string();
    modified_path = modified_path.chars().rev().collect::<String>();

    // Writing into the new file.
    export_content_into_file(&format!("{}{}.html", &directory, &modified_path), &content);

    println!("{} rendered into {}.html\n", &path, &modified_path)
}