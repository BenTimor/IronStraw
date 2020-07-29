use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::processing::full_process;
use crate::config::get_config;
use crate::utils::get_blocks_as_content;

/// Loop allows you to run a statement multiple times
pub struct LoopCommand {}

impl Command for LoopCommand {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>) -> String {
        // The 'text' of the command is the amount of times that the loop has to run
        let times: usize = text.parse::<usize>()
            .expect(&*format!("The text {} is not a valid number!", text));

        // Taking the blocks, turning them into a String with the same amount of spaces in the start.
        let content: String = get_blocks_as_content(&blocks);

        let mut result: Vec<String> = Vec::new();

        // Running the content {times} amount of times
        for _ in 0..times {
            result.push(full_process(&content, &get_config(false)));
        }

        result.join("\n")
    }
}