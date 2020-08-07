use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::config::Config;

pub struct StrawNote {}

/// Does nothing. Useful for notes.
impl Command for StrawNote {
    fn run(&self, _command: &String, _parameters: &Vec<String>, _text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>, _config: &Config) -> String {
        "".to_string()
    }
}