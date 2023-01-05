use clap::Parser;
use commands::Binocular;
use std::{
    ffi::OsStr,
    process::Command,
};

use crate::cli::{Cli, ParsedCli};

mod consts;
mod commands;
mod cli;

fn fzf_command(binocular: Binocular, cli: &ParsedCli) -> Command {
    let grep_cmd = binocular.parse_grep_command();
    let file_cmd = binocular.parse_file_command();
    let folder_cmd = binocular.parse_folder_command();

    let (mut cmd, prompt, preview) = match cli.mode {
        cli::ModeEnum::Grep => (binocular.grep_command, consts::RG_PROMPT, consts::RG_PREVIEW),
        cli::ModeEnum::File => (binocular.file_command, consts::FILES_PROMPT, consts::FILES_PREVIEW),
        cli::ModeEnum::Directory => (binocular.folder_command, consts::FOLDER_PROMPT, consts::FOLDER_PREVIEW),
    };

    let (open_editor, open_new_editor) = match cli.shortcut_editor {
        cli::EditorEnum::Code => (consts::VSCODE_EDITOR_COMMAND, consts::VSCODE_EDITOR_COMMAND_NEW_WINDOW),
        cli::EditorEnum::Insiders => (consts::INSIDERS_EDITOR_COMMAND, consts::INSIDERS_EDITOR_COMMAND_NEW_WINDOW),
        cli::EditorEnum::Vim => (consts::VSCODE_EDITOR_COMMAND, consts::VSCODE_EDITOR_COMMAND_NEW_WINDOW),
        cli::EditorEnum::Explorer => (consts::VSCODE_EDITOR_COMMAND, consts::VSCODE_EDITOR_COMMAND_NEW_WINDOW),
        cli::EditorEnum::Echo => (consts::VSCODE_EDITOR_COMMAND, consts::VSCODE_EDITOR_COMMAND_NEW_WINDOW),
    };

    let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");
    
    let mut args: Vec<&OsStr> = consts::FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();
    
    let prompt_line = format!("--prompt={}", prompt);
    let open_editor_line = format!("--bind=ctrl-o:execute-silent({})+abort", open_editor);
    let open_editor_new_line = format!("--bind=ctrl-n:execute-silent({})+abort", open_new_editor);
    let grep_line = format!("--bind=ctrl-g:reload({} {{q}})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-r)+rebind(change,ctrl-f)+rebind(change,ctrl-d)", grep_cmd, consts::RG_PROMPT, consts::RG_PREVIEW);
    let file_line = format!("--bind=ctrl-f:reload({})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-f)+rebind(change,ctrl-r)+rebind(change,ctrl-d)", file_cmd, consts::FILES_PROMPT, consts::FILES_PREVIEW);
    let directory_line = format!("--bind=ctrl-d:reload({})+change-prompt({})+change-preview-window(hidden)+change-preview({})+unbind(change,ctrl-d)+rebind(change,ctrl-r)+rebind(change,ctrl-f)", folder_cmd, consts::FOLDER_PROMPT, consts::FOLDER_PREVIEW);
    let preview_line = format!("--preview={}", preview);
    let query_line = format!("-q {}", &cli.query);

    args.push(OsStr::new(&prompt_line));
    args.push(OsStr::new(&open_editor_line));
    args.push(OsStr::new(&open_editor_new_line));
    args.push(OsStr::new(&grep_line));
    args.push(OsStr::new(&file_line));
    args.push(OsStr::new(&directory_line));
    args.push(OsStr::new(&preview_line));
    args.push(OsStr::new(&query_line));

    let mut fzf = Command::new("fzf");
    fzf.args(args).stdin(std_out);
    return fzf;
}

fn main() {
    let cli = Cli::parse();
    let parsed_cli = ParsedCli::new(&cli);
    println!("{:?}", parsed_cli);

    let binocular = commands::Binocular::new(&parsed_cli.path);
    println!("{}", binocular.parse_grep_command());

    let fzf = fzf_command(binocular, &parsed_cli)
        .spawn()
        .unwrap();

    let output = fzf.wait_with_output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap();
    println!("{}", result);
}