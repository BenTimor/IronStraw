use crate::config::Config;

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
                commands.push(Box::new(PreprocessedObject::Block {text: line, spaces: current_spaces}));
                continue;
            }
            else {
                block_spaces = 0;
            }
        }

        // Checking if it's a short command. If it is, then replace it with the long command.
        for (short_command, long_command) in config.short_commands.clone() {
            if line.starts_with(&short_command) {
                line = line.replacen(&short_command[..], &format!("{} ", long_command)[..], 1);
                break;
            }
        }

        // If it's a command, start a block
        if line.starts_with("@") {
            block_spaces = current_spaces;
        }

        commands.push(Box::new(PreprocessedObject::Command {
            command: get_command(&line),
            parms: get_parameters(&line),
            text: lstrip(&get_text(&line).unwrap_or("".to_string())),
            spaces: current_spaces
        }));

    }

    commands
}

/// In the original Straw (which is written in Python) I've used two different classes.
/// Because Rust don't really like the using of two different classes in a Vec, I've created an enum.
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