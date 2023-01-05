use clap::Parser;
use std::{
    ffi::OsStr,
    process::Command,
};

use crate::cli::{Cli, ParsedCli};

mod consts;
mod commands;
mod cli;

fn fzf_command(mut cmd: Command, query: &String) -> Command {
    let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");
    
    let q = "-q ".to_owned()+query;

    let mut args: Vec<&OsStr> = consts::FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();

    args.push(OsStr::new("--prompt=updatemeeeeeeeee"));
    args.push(OsStr::new("--bind=ctrl-o:execute-silent($$$COMMAND-TO-OPEN-VSCODE$$$)+abort"));
    args.push(OsStr::new("--bind=ctrl-n:execute-silent($$$COMMAND-TO-OPEN-VSCODE-NEW-WINDOW$$$)+abort"));
    args.push(OsStr::new("--bind=ctrl-g:reload($GREP_CMD {q})+change-prompt($GREP_PROMPT)+change-preview-window(50%)+change-preview($GREP_PREVIEW_STYLE)+unbind(change,ctrl-r)+rebind(change,ctrl-f)+rebind(change,ctrl-d)"));
    args.push(OsStr::new("--bind=ctrl-f:reload($FILE_CMD)+change-prompt($FILE_PROMPT)+change-preview-window(50%)+change-preview($FILE_PREVIEW_STYLE)+unbind(change,ctrl-f)+rebind(change,ctrl-r)+rebind(change,ctrl-d)"));
    args.push(OsStr::new("--bind=ctrl-d:reload($DIRECTORY_CMD)+change-prompt($DIRECTORY_PROMPT)+change-preview-window(hidden)+change-preview($DIRECTORY_PREVIEW_STYLE)+unbind(change,ctrl-d)+rebind(change,ctrl-r)+rebind(change,ctrl-f)"));
    args.push(OsStr::new("--preview=bat --color=always {1} --highlight-line {2}"));
    args.push(OsStr::new(&q));
    // args.push(OsStr::new("-q"));
    // args.push(OsStr::new(query));

    let mut fzf = Command::new("fzf");
    fzf.args(args).stdin(std_out);
    return fzf;
}

fn main() {
    let cli = Cli::parse();
    let parsed_cli = ParsedCli::new(&cli);
    println!("{:?}", parsed_cli);

    let commands = commands::Binocular::new(&parsed_cli.path);
    println!("{}", commands.parse_grep_command());

    let fzf = fzf_command(commands.grep_command, &parsed_cli.query)
        .spawn()
        .unwrap();

    let output = fzf.wait_with_output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap();
    println!("{}", result);
}