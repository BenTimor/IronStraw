use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::processing::full_process;
use crate::config::{get_config, Config};
use crate::utils::get_blocks_as_content;

/// Loop allows you to run a statement multiple times
/// @loop TIMES
pub struct LoopCommand {}

impl Command for LoopCommand {
    fn run(&self, _command: &String, parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>, config: &Config) -> String {
        // The 'text' of the command is the amount of times that the loop has to run
        let times: usize = text.parse::<usize>()
            .unwrap_or_else(|_| {
                println!("The text {} in @loop command is not a valid number! Skipping.", text);
                0
            });

        // Taking the blocks, turning them into a String with the same amount of spaces in the start.
        let content: String = get_blocks_as_content(&blocks);
        let var = parameters.get(0);
        let mut result: Vec<String> = Vec::new();

        // Running the content {times} amount of times
        for index in 0..times {
            let mut processed = full_process(&content, &get_config(false, config.debug));
            if var.is_some() {
                processed = processed.replace(var.unwrap(), (index+1).to_string().as_str());
            }
            result.push(processed);
        }

        result.join("\n")
    }
}