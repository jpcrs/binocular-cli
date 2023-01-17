use std::{process::Command, path::PathBuf};

use crate::helper;

#[allow(dead_code)]
const PREVIEW: &str = "bat --color=always {}README.md";

#[allow(dead_code)]
const PROMPT: &str = "Git > ";

#[allow(dead_code)]
const PARAMS: &'static [&str] = &[
    "--glob",
    ".git",
    "-td", 
    "-H",
    "--exclude",
    "node_modules"
];

pub struct GitFolders {
    pub command: Command,
    pub preview: String,
    pub prompt: String
}

#[allow(dead_code)]
impl GitFolders {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        GitFolders {
            command: helper::init_command(paths, "fd", PARAMS),
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