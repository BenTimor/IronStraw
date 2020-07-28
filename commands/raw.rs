use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use std::ops::Deref;

/// Allows you to use HTML "as is"
/// For example, @raw <b> Hey </b> will return <b> Hey </b>
pub struct Raw {}

impl Command for Raw {
    fn run(&self, _command: &String, _parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>) -> String {
        let mut raw: Vec<String> = Vec::new();
        raw.push(text.clone());
        for block in blocks {
            if let PreprocessedObject::Block { text, spaces: _ } = block.deref() {
                raw.push(text.clone());
            }
        }
        raw.join("\n")
    }
}