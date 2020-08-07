use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use std::ops::Deref;
use crate::config::Config;

/// Note command allows you to create HTML notes
/// @note Optional:NOTE
///     Optional:NOTE
pub struct Note {}

impl Command for Note {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>, _config: &Config) -> String {
        let mut note: Vec<String> = Vec::new();
        note.push("<!--".to_string());
        note.push(text.clone());
        for block in blocks {
            if let PreprocessedObject::Block { text, spaces: _ } = block.deref() {
                note.push(text.clone());
            }
        }
        note.push("-->".to_string());
        note.join("\n")
    }
}