use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use std::fs::remove_file;

/// Allows you to remove a file
/// @delfile PATH
pub struct DelFile {}

impl Command for DelFile {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>) -> String {
        if remove_file(text).is_err() {
            println!("Couldn't delete the file {} with the command @delfile", &text)
        }
        "".to_string()
    }
}