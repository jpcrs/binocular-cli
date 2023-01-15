/*
This is the code of someone who tried to challenge the Borrow Checker without previously learning Rust.

I started throwing code, during the battle, after immense suffering, I assumed defeat and gave up. Someday I'll refactor it, this is a promise, don't take this code seriously.
*/
use clap::Parser;
use commands::default_fzf::exec_fzf;
use commands::file_content::FileContent;
use commands::file_name::FileName;
use commands::folder_name::FolderName;
use commands::history_fzf::exec_history_fzf;
use commands::history_rg::HistoryRg;
use crate::cli::{Cli, ParsedCli, SubCommands};

mod cli;
mod helper;
mod commands;

fn main() {
    let cli = Cli::parse();
    let parsed_cli = ParsedCli::new(&cli);

    if let Some(history) = parsed_cli.history {
        match history {
            SubCommands::History { path, file } => {
                let file_content = &mut FileContent::new(&parsed_cli.path);
                let history_rg = &mut HistoryRg::new(path, file);
                exec_history_fzf(file_content, history_rg, &parsed_cli);
            }
        }
    }
    else {
        // Everything mutable because Command has to be mutable so I can spawn it.
        // I'm not ready to use RefCell or whatever.
        let file_content = &mut FileContent::new(&parsed_cli.path);
        let file_name = &mut FileName::new(&parsed_cli.path);
        let folder_name = &mut FolderName::new(&parsed_cli.path);
        exec_fzf(file_content, file_name, folder_name, &parsed_cli)
    }
}