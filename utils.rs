use std::path::Path;
use std::fs::File;
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