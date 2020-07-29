use crate::preprocessing::PreprocessedObject;
use std::process::Command as TerminalCommand;
use crate::commands::Command;

/// Allows you to run terminal commands
pub struct Terminal {}

impl Command for Terminal {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>) -> String {
        // Literally copied pasted from several stackoverflow posts
        let output_option = TerminalCommand::new("sh").arg("-c").arg(text).output().unwrap();
        String::from_utf8_lossy(&output_option.stdout).parse().unwrap()
    }
}