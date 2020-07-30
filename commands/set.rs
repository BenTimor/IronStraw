use crate::commands::PreprocessedCommand;
use crate::preprocessing::PreprocessedObject;
use std::ops::Deref;

/// Allows you to create "variables" by replacing whatever you choose in the file
/// ^set(from, to)
pub struct Set {}

impl PreprocessedCommand for Set {
    fn run(&self, _command: &String, parameters: &Vec<String>, _text: &String, _spaces: &usize, _blocks: &Vec<Box<PreprocessedObject>>, preprocessed: Vec<Box<PreprocessedObject>>) -> Vec<Box<PreprocessedObject>> {
        let from = parameters.get(0).expect("You have to enter parameters for ^set command");
        let to = parameters.get(1).expect("You have to enter two parameters for ^set command");

        // Doing map to change the content of the vector
        // Just copying anything and replacing whatever the parameter say
        preprocessed.into_iter().map(|f| {
            match f.deref() {
                PreprocessedObject::Command { command, parms, text, spaces } => {
                    Box::new(PreprocessedObject::Command {
                        command: command.replace(from, to),
                        parms: parms.into_iter().map(|f| f.replace(from, to)).collect::<Vec<String>>(),
                        text: text.replace(from, to),
                        spaces: spaces.clone()
                    })
                },
                PreprocessedObject::Block { text, spaces } => {
                    Box::new(PreprocessedObject::Block {
                        text: text.replace(from, to),
                        spaces: spaces.clone()
                    })
                }
            }
        }).collect::<Vec<Box<PreprocessedObject>>>()
    }
}