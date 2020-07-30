use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use std::fs::remove_file;

/// Allows you to remove a file
/// @delfile PATH
struct DelFile {}

impl Command for DelFile {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>) -> String {
        remove_file(text).expect(&*format!("Couldn't delete the file {}", &text));
        "".to_string()
    }
}