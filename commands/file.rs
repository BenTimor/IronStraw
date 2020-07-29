use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::processing::full_process;
use crate::config::get_config;
use crate::utils::get_file_content;

/// Allows you to read a file. If the first parameter is "true", it'll also process the content.
pub struct File {}

impl Command for File {
    fn run(&self, _command: &String, parameters: &Vec<String>, text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>) -> String {
        // Check of the first parameter is "true"
        let process: bool = parameters.get(0).unwrap_or(&"false".to_string()).eq("true");
        let content: String = get_file_content(text);

        if process {
            return full_process(&content, &get_config(false));
        }
        content
    }
}