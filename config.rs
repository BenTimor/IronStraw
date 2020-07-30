use std::collections::HashMap;
use crate::commands::{Command, setup_commands, PreprocessedCommand};

pub struct Config {
    pub commands: HashMap<String, Box<dyn Command>>,
    pub preprocessed_commands: HashMap<String, Box<dyn PreprocessedCommand>>,
    pub short_commands: HashMap<String, String>,
    pub add_html: bool
}

pub fn get_config(add_html: bool) -> Config {
    let mut config = Config {
        commands: HashMap::new(),
        preprocessed_commands: HashMap::new(),
        short_commands: HashMap::new(),
        add_html
    };

    config.short_commands.insert("//".to_string(), "@straw_note".to_string());
    config.short_commands.insert(".".to_string(), "@raw".to_string());
    setup_commands(&mut config);


    config
}