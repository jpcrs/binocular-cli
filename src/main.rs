/*
This is the code of someone who tried to challenge the Borrow Checker without previously learning Rust.

I started throwing code, during the battle, after immense suffering, I assumed defeat and gave up. Someday I'll refactor it, this is a promise, don't take this code seriously.
*/
use clap::Parser;

use crate::cli::{Cli, ParsedCli, SubCommands};

#[path = "commands/default_fzf.rs"]
mod default_fzf;

#[path = "commands/file_content.rs"]
mod file_content;

#[path = "commands/file_name.rs"]
mod file_name;

#[path = "commands/folder_name.rs"]
mod folder_name;

#[path = "commands/open_editor.rs"]
mod open_editor;

#[path = "commands/history_rg.rs"]
mod history_rg;

#[path = "commands/history_fzf.rs"]
mod history_fzf;

mod cli;
mod helper;

fn main() {
    let cli = Cli::parse();
    let parsed_cli = ParsedCli::new(&cli);

    if let Some(history) = parsed_cli.history {
        match history {
            SubCommands::History { path, file } => {
                let file_content = &mut file_content::FileContent::new(&parsed_cli.path);
                let history_rg = &mut history_rg::HistoryRg::new(path, file);
                history_fzf::history_fzf_commmand(file_content, history_rg, &parsed_cli);
            }
        }
    }
    else {
        let file_content = &mut file_content::FileContent::new(&parsed_cli.path);
        let file_name = &mut file_name::FileName::new(&parsed_cli.path);
        let folder_name = &mut folder_name::FolderName::new(&parsed_cli.path);
        default_fzf::fzf_command2(file_content, file_name, folder_name, &parsed_cli)
    }
}