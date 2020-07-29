mod terminal;
mod intofile;
mod file;
mod raw;
mod note;
mod straw_note;
mod loop_command;

use crate::config::Config;
use crate::preprocessing::PreprocessedObject;
use crate::commands::straw_note::StrawNote;
use crate::commands::raw::Raw;
use crate::commands::note::Note;
use crate::commands::loop_command::LoopCommand;
use crate::commands::file::File;
use crate::commands::intofile::IntoFile;
use crate::commands::terminal::Terminal;

/// This trait is used to create new commands.
pub trait Command {
    fn run(
        &self,
        command: &String,
        parameters: &Vec<String>,
        text: &String, spaces: &usize,
        blocks: &Vec<Box<PreprocessedObject>>
    ) -> String;
}

pub fn setup_commands(config: &mut Config) {

    config.short_commands.insert("//".to_string(), "@straw_note".to_string());
    config.commands.insert("@straw_note".to_string(), Box::new(StrawNote{}));

    config.short_commands.insert(".".to_string(), "@raw".to_string());
    config.commands.insert("@raw".to_string(), Box::new(Raw{}));

    config.short_commands.insert("#".to_string(), "@note".to_string());
    config.commands.insert("@note".to_string(), Box::new(Note{}));

    config.commands.insert("@loop".to_string(), Box::new(LoopCommand{}));

    config.commands.insert("@file".to_string(), Box::new(File{}));

    config.commands.insert("@intofile".to_string(), Box::new(IntoFile{}));

    config.commands.insert("@terminal".to_string(), Box::new(Terminal{}));

}