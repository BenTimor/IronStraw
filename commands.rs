mod raw;
mod note;
mod straw_note;

use crate::config::Config;
use crate::commands::straw_note::StrawNote;
use crate::commands::raw::Raw;
use crate::commands::note::Note;
use crate::preprocessing::PreprocessedObject;

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

}