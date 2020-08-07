use crate::config::Config;
use std::ops::Deref;
use crate::preprocessing::PreprocessedObject::Block;
use crate::utils::debug;

/// This method is the main preprocessing method.
/// It takes a content of file and converts it into PreprocessedObjects.
/// Later, The process method will take the objects and transfer them into a HTML code.
pub fn preprocess<'a>(content: &String, config: &Config) -> Vec<Box<PreprocessedObject>> {
    // This is the content after breaking to lines, maybe adding html and adding one space before everything
    let formatted_content = format_content(&content, config.add_html);
    // In this Vec we're going to have our Command/Block objects
    let mut commands: Vec<Box<PreprocessedObject>> = Vec::new();
    // If we're in a block, this var is going to have the spaces amount of the block
    let mut block_spaces: usize = 0;

    // We're looping through the content and converting every line to Command/Block
    for index_line in formatted_content {
        let current_spaces: usize = count_spaces(&index_line);
        let mut line: String = lstrip(&index_line);

        // Skipping blank lines
        if line.is_empty() {
            continue;
        }

        // If we're in a block, add a Block object. // Look at the 'current_spaces' variable note.
        if block_spaces != 0 {
            if current_spaces > block_spaces {
                debug(format!("preprocessing.rs::preprocess saving a block | spaces={} | text={}", &current_spaces, &line), &config);
                commands.push(Box::new(PreprocessedObject::Block {text: line, spaces: current_spaces}));
                continue;
            }
            else {
                debug(format!("preprocessing.rs::preprocess getting out of a block"), &config);
                block_spaces = 0;
            }
        }

        // Checking if it's a short command. If it is, then replace it with the long command.
        for (short_command, long_command) in config.short_commands.clone() {
            if line.starts_with(&short_command) {
                debug(format!("preprocessing.rs::preprocess replacing short command {} with {}", &short_command, &long_command), &config);
                line = line.replacen(&short_command[..], &format!("{} ", long_command)[..], 1);
                break;
            }
        }

        // If it's a command, start a block
        if line.starts_with("@") {
            debug(format!("preprocessing.rs::preprocess entering into the block of line={}", &line), &config);
            block_spaces = current_spaces;
        }

        commands.push(Box::new(PreprocessedObject::Command {
            command: get_command(&line),
            parms: get_parameters(&line),
            text: lstrip(&get_text(&line).unwrap_or("".to_string())),
            spaces: current_spaces
        }));

    }

    // We have to add one more command so it'll run through everything
    debug("preprocessing.rs::preprocess adding an empty command so it'll add one more iterate".to_string(), &config);
    commands.push(Box::new(PreprocessedObject::Command {
        command: "@".to_string(),
        parms: vec![],
        text: "".to_string(),
        spaces: 0
    }));

    debug("preprocessing.rs::preprocess starting the preprocessed commands".to_string(), &config);
    let mut i = 0;
    let mut blocks: Vec<Box<PreprocessedObject>> = Vec::new();
    let mut optional_last_command: Option<Box<PreprocessedObject>> = Option::None{};
    let mut temporary_commands: Vec<Box<PreprocessedObject>> = Vec::new();
    // The loop is running until it doesn't find any element. It does it in case some command will change the commands vector size.
    loop {
        if !temporary_commands.is_empty() {
            debug("preprocessing.rs::preprocess moving the temporary_commands into the commands".to_string(), &config);
            commands = temporary_commands.clone();
            temporary_commands = Vec::new();
        }
        let optional_object = commands.get(i);

        match optional_object {
            // If we're at the end of the vector. break.
            Option::None {} => {
                break;
            },
            Option::Some(object) => {
                match object.deref() {
                    // If it's a block, save it for the command. Very similar to process method.
                    PreprocessedObject::Block { text, spaces } => {
                        blocks.push(Box::new(Block {text: text.clone(), spaces: spaces.clone()}));
                    },
                    PreprocessedObject::Command { command, parms: _, text: _, spaces: _ } => {
                        // If it's a command and there is a last command:
                        // it means that we've finished to loop through the blocks of the last command, so we have to run it.
                        if let Option::Some(last_command) = &optional_last_command {
                            if let PreprocessedObject::Command { command, parms, text, spaces } = last_command.deref() {
                                let command_object = config.preprocessed_commands.get(command);

                                if command_object.is_none() {
                                    println!("The preprocessed command {} is not found. Skipping.", &command);
                                    i=i+1;
                                    optional_last_command = Option::None;
                                    continue;
                                }

                                debug(format!("preprocessing.rs::preprocess running the command {}", &command), &config);
                                temporary_commands = command_object.unwrap()
                                    .run(&command, &parms, &text, &spaces, &mut blocks, commands.clone(), &config);
                            }
                            optional_last_command = Option::None;
                        }

                        // If it is a command which starts with "^", it's a preprocess command, so run it.
                        if command.starts_with("^") {
                            optional_last_command = Option::Some(object.clone());
                        }
                    }
                }
            }
        }

        i = i+1;
    }

    commands
}

/// In the original Straw (which is written in Python) I've used two different classes.
/// Because Rust don't really like the using of two different classes in a Vec, I've created an enum.
#[derive(Debug, Clone)]
pub enum PreprocessedObject {
    Command{command: String, parms: Vec<String>, text: String, spaces: usize},
    Block{text: String, spaces: usize}
}


/// Removes the spaces from the start and the end of the text.
fn strip(text: &String) -> String {
    let mut new_text: String = text.clone();
    new_text = rstrip(&new_text);
    new_text = lstrip(&new_text);
    new_text
}

/// Removes the spaces from the end of the text.
fn rstrip(text: &String) -> String {
    let mut reversed_string: String = text.chars().rev().collect();
    reversed_string = lstrip(&reversed_string);
    reversed_string.chars().rev().collect()
}

/// Removes the spaces from the start of the text.
fn lstrip(text: &String) -> String {
    let mut new_string: String = String::new();
    let mut past_spaces = false;

    for c in text.chars() {
        if !past_spaces && c != ' ' {
            past_spaces = true;
        }

        if past_spaces {
            new_string.push(c);
        }
    }

    new_string
}

/// Counting the amount of spaces in the start of the text.
/// It allows the preprocess to know what block it is.
fn count_spaces(text: &String) -> usize {
    text.chars().count() - lstrip(text).chars().count()
}

/// This method strips tabs and adds a "HTML" tag (if needed)
fn format_content(content: &String, addhtml: bool) -> Vec<String> {
    // Checks if it has to add "html" tag. If so, it appends the tag.
    let mut lines: Vec<String> =
        if addhtml {
            vec![".<!DOCTYPE html>".to_string() ,"html".to_string()]
        } else {
            Vec::new()
        };

    // Adding the rest of the content to the Vec we created before.
    lines.extend::<Vec<String>>(
        content.lines().map(
            |line|
                // Converting tabs to spaces and adding space to the start of everything (so it'll be in the HTML tag).
                format!(" {}", &line.replace("\t", " ")))
            .collect()
    );

    lines
}

/// Used to decide if there are parameters in a command.
fn has_parameters(line: &String) -> bool {
    // If space comes first, there are not parms.
    // If '(' comes first, there are parms.
    for c in line.chars() {
        match c {
            ' ' => return false,
            '(' => return true,
            _ => {}
        }
    }

    return false;
}

/// It's used to get the parameters of a command.
fn get_parameters(line: &String) -> Vec<String> {
    // If no parameters, return an empty Vec
    if !has_parameters(line) {
        return Vec::new();
    }

    // Splits the command itself and the parameters
    line.splitn(2, "(").nth(1).unwrap_or("NONE)NONE").
        // Splits the parameters and the text
        split(")").nth(0).unwrap_or("NONE,NONE").
        // Splits the parameters to Vec and strips them.
        split(",").map(|f| strip(&f.to_string())).collect()
}

/// Gets the command itself. For example, in @command(parms) HEY it'll return @command.
fn get_command(line: &String) -> String {
    line.split(if has_parameters(line) { "(" } else { " " }).nth(0).unwrap().to_string()
}

/// Returns the text of a command.
/// For example, @command(parms) text, will return "text"
fn get_text(line: &String) -> Option<String> {
    line.splitn(2, if has_parameters(line) { ")" } else { " " }).nth(1).map(|f| f.to_string())
}