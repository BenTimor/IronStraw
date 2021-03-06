use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::utils::{get_blocks_as_content, export_content_into_file};
use crate::processing::full_process;
use crate::config::{get_config, Config};

/// Allows you to write into files.
/// @intofile(process_content: bool DEFAULT=false, add_html: bool DEFAULT=false)
///     CONTENT
pub struct IntoFile {}

impl Command for IntoFile {
    fn run(&self, _command: &String, parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>, config: &Config) -> String {
        // Check of the first parameter is "true"
        let process: bool = parameters.get(0).unwrap_or(&"false".to_string()).eq("true");
        let html: bool = parameters.get(1).unwrap_or(&"false".to_string()).eq("true");
        let content: String = get_blocks_as_content(&blocks);

        if process {
            export_content_into_file(&text, &full_process(&content, &get_config(html, config.debug)));
        } else {
            export_content_into_file(&text, &content);
        }

        "".to_string()
    }
}