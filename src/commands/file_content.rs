use std::{path::PathBuf, process::Command};

use crate::helper;

#[allow(dead_code)]
const PREVIEW: &str = "bat --color=always {1} --highlight-line {2} --style=header,numbers";

#[allow(dead_code)]
const PROMPT: &str = "  Grep  ";
#[allow(dead_code)]

const PARAMS: &'static [&str] = &[
    "--hidden",
    "--column",
    "--line-number",
    "--no-heading",
    "--smart-case",
    "",
];

#[allow(dead_code)]
pub struct FileContent {
    pub command: Command,
    pub preview: String,
    pub prompt: String,
}

#[allow(dead_code)]
impl FileContent {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        FileContent {
            command: helper::init_command(paths, "rg", PARAMS),
            preview: String::from(PREVIEW),
            prompt: String::from(PROMPT),
        }
    }

    pub fn get_command_info(&mut self) -> (&mut Command, &'static str, &'static str) {
        (&mut self.command, PROMPT, PREVIEW)
    }

    pub fn parse_command(&self) -> String {
        return helper::get_command_string(&self.command);
    }
}
