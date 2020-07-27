use std::collections::HashMap;
use crate::preprocessing::PreprocessedObject;
use std::ops::Deref;

pub struct Config {
    pub processed_commands: HashMap<String, Box<dyn Fn(&String, &Vec<String>, &String, &usize, &Vec<Box<PreprocessedObject>>) -> String>>,
    pub short_commands: HashMap<String, String>,
    pub add_html: bool
}

pub fn get_config(add_html: bool) -> Config {
    let mut config = Config {
        processed_commands: HashMap::new(),
        short_commands: HashMap::new(),
        add_html
    };

    config.short_commands.insert("//".to_string(), "@straw_note".to_string());
    config.short_commands.insert(".".to_string(), "@raw".to_string());

    config.processed_commands.insert("@straw_note".to_string(), Box::new(
        |_, _, _, _, _| {
            "".to_string()
    }));

    config.processed_commands.insert("@raw".to_string(), Box::new(
        |_, _, text, _, blocks| {
            let mut raw: Vec<String> = Vec::new();
            raw.push(text.clone());
            for block in blocks {
                if let PreprocessedObject::Block { text, spaces: _ } = block.deref() {
                    raw.push(text.clone());
                }
            }
            raw.join("\n")
    }));

    config.processed_commands.insert("@note".to_string(), Box::new(
        |_, _, text, _, blocks| {
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
    }));

    config
}