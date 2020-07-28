use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;

pub struct StrawNote {}

/// Does nothing. Useful for notes.
impl Command for StrawNote {
    fn run(&self, _command: &String, _parameters: &Vec<String>, _text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>) -> String {
        "".to_string()
    }
}