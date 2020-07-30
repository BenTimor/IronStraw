use crate::preprocessing::{PreprocessedObject, preprocess};
use crate::config::Config;
use std::ops::Deref;

/// This method takes the preprocess method and the process method and returns the final result
pub fn full_process(content: &String, config: &Config) -> String {
    process(preprocess(&content, &config), &config)
}

/// The process method is taking the result of the preprocessed method and converting it into HTML
fn process(preprocessed: Vec<Box<PreprocessedObject>>, config: &Config) -> String {
    // The blocks of the last command
    let mut blocks: Vec<Box<PreprocessedObject>> = Vec::new();
    // The last command. So when we finish to add blocks, we can run it.
    let mut last_command: Option<Box<PreprocessedObject>> = Option::None;
    // The lines of the HTML file.
    let mut processed_content: Vec<String> = Vec::new();
    let mut html_commands: Vec<Box<PreprocessedObject>> = Vec::new();

    // Looping through the preprocessed content
    for object in preprocessed {
        // Checking if it's a Command or a Block
        match object.deref() {
            // If it's a command...
            PreprocessedObject::Command { command, parms, text, spaces } => {
                // If there's a last command and we got a new command, run the last one.
                if let Option::Some(some_last_command) = &last_command {
                    // Really just to get the values
                    if let PreprocessedObject::Command { command, parms, text, spaces } = some_last_command.deref() {

                        // If it's a preprocess command, we already ran it in the preprocessing method.
                        if command.starts_with("^") {
                            blocks = Vec::new();
                            last_command = Option::None;
                            continue;
                        }

                        let result = config.commands.get(command)
                        .expect(&*format!("The command {} doesn't exist", command))
                            .run(&command, &parms, &text, &spaces, &mut blocks);
                        processed_content.push(result);
                        blocks = Vec::new();
                        last_command = Option::None;
                    }
                }

                // If the command starts with '@' we want to save it as a last command. This is because we want to run it after.
                // Update 30.07.2020: I've added the preprocessed command and I want it to find its blocks so I can ignore both when running the commands.
                if command.starts_with("@") || command.starts_with("^") {
                    close_html_tags(&mut html_commands, &mut processed_content, &spaces);
                    last_command = Option::Some(object);
                    continue;
                }

                // Close what you have to close and than open the tag
                close_html_tags(&mut html_commands, &mut processed_content, &spaces);
                processed_content.push(format!("<{} {}> {}", command, parms.join(" "), text));
                html_commands.push(object);
            },
            PreprocessedObject::Block { text: _, spaces: _ } => {
                blocks.push(object);
            }
        }
    }

    close_html_tags(&mut html_commands, &mut processed_content, &0);

    processed_content.join("\n")
}

/// Closing the HTML tags (which are out of the current block)
fn close_html_tags(html_commands: &mut Vec<Box<PreprocessedObject>>, processed_content: &mut Vec<String>, current_spaces: &usize) {
    // It's all to loop backwards (the last one to open, the first to close...)
    let mut index: usize = (&html_commands).len();
    while index != 0 {
        index = index-1;
        // Extract values from Command enum
        if let PreprocessedObject::Command { command, parms: _, text: _, spaces } = (&html_commands).get(index).unwrap().deref() {
            // If it's out of the block, close
            if current_spaces <= spaces {
                processed_content.push(format!("</{}>", &command));
                html_commands.remove(index);
            }
            // If it's not out of the block, there is nothing more to close. Break the loop.
            else {
                break;
            }
        }
    }
}