use std::path::Path;
use std::fs::{File, create_dir_all, metadata};
use std::io::{Write, Read};
use crate::preprocessing::PreprocessedObject;
use std::ops::Deref;

/// Returns the content of a file
pub fn get_file_content(path: &String) -> String {
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
pub fn export_content_into_file(path: &String, content: &String) {
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

/// Returning the blocks like it was still a text in the file
pub fn get_blocks_as_content(blocks: &Vec<Box<PreprocessedObject>>) -> String {
    blocks.into_iter()
        .map(|f| {
            if let PreprocessedObject::Block {text, spaces} = f.deref() {
                return format!("{}{}", " ".repeat(*spaces), text.clone());
            }
            "".to_string()
        })
        .collect::<Vec<String>>().join("\n")
}

/// Creating a directory if it's not exist.
/// If there's an error, it's printing to the console.
pub fn create_directory_if_not_exist(directory: &String) {
    let directory_metadata = metadata(&directory);
    if directory_metadata.is_err() || directory_metadata.unwrap().is_file() {
        if create_dir_all(&directory).is_err() {
            println!("Couldn't create the directory {}", &directory);
            return;
        }
    }
}

pub fn get_argument_parameter(arg: &String, args: &mut Vec<String>) -> Option<String> {
    // Find target arg, if there's one
    let index = args.iter().position(|x| x.eq_ignore_ascii_case(&arg));
    let mut result: Option<String> = Option::None;
    // Get the arg itself and remove these args from the vector
    if index.is_some() {
        result = Option::Some(args.get(index.unwrap()+1).unwrap().clone());
        args.remove(index.unwrap());
        args.remove(index.unwrap());
    }
    result
}