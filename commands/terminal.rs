use crate::preprocessing::PreprocessedObject;
use std::process::Command as TerminalCommand;
use crate::commands::Command;
use crate::config::Config;

/// Allows you to run terminal commands
pub struct Terminal {}

impl Command for Terminal {
    fn run(&self, _command: &String, parameters: &Vec<String>, text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>, _config: &Config) -> String {
        // Literally copied pasted from several stackoverflow posts
        let return_output = parameters.get(0).unwrap_or(&"true".to_string()).eq_ignore_ascii_case("true");
        let output_option = TerminalCommand::new("sh").arg("-c").arg(text).output().unwrap();
        if return_output {
            return String::from_utf8_lossy(&output_option.stdout).parse().unwrap();
        }
        "".to_string()
    }
}