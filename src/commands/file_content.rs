use std::{process::Command, path::PathBuf};

use crate::helper;

#[allow(dead_code)]
const PREVIEW: &str = "bat --color=always {1} --highlight-line {2}";

#[allow(dead_code)]
const PROMPT: &str = "Grep > ";
#[allow(dead_code)]

const PARAMS: &'static [&str] = &[
    "--hidden",
    "--column",
    "--line-number",
    "--no-heading",
    "--color=always",
    "--smart-case",
    ""
];


#[allow(dead_code)]
pub struct FileContent {
    pub command: Command,
    // pub parsed_command: String,
    pub preview: String,
    pub prompt: String
}


#[allow(dead_code)]
impl  FileContent {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        FileContent {
            command: helper::init_command(paths, "rg", PARAMS),
            // parsed_command: helper::get_command_string(cmd),
            preview: String::from(PREVIEW),
            prompt: String::from(PROMPT)
        }
    }

    pub fn get_command_info(&mut self) -> (&mut Command, &'static str, &'static str) {
        (&mut self.command, PROMPT, PREVIEW)
    }

    pub fn parse_command(&self) -> String {
        return helper::get_command_string(&self.command);
    }
}