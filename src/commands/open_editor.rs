use std::{process::{ChildStdout, Command}, io::Read};

use crate::{helper, cli::{ParsedCli, self}};

pub const VSCODE_NEW_WINDOW: &str ="code -n -g {1}:{2}";
pub const INSIDERS_NEW_WINDOW: &str ="code-insiders -n -g {1}:{2}";

pub const VSCODE_APPEND_WINDOW: &str ="code -a -g {1}:{2}";
pub const INSIDERS_APPEND_WINDOW: &str ="code-insiders -a -g {1}:{2}";

pub fn open_on_editor(stdout: &mut ChildStdout, cli: &ParsedCli) {
    let mut selected = String::new();
    stdout.read_to_string(&mut selected).expect("failed to read from stdout");

    if selected.is_empty()
    {
        return;
    }

    let res = helper::get_file_and_line(selected);

    let binding_code= &mut Command::new("code");
    let binding_insiders = &mut Command::new("code-insiders");
    let cmd = match cli.editor {
        cli::EditorEnum::Code => binding_code.arg("-r").arg("-g").arg(res),
        cli::EditorEnum::Insiders => binding_insiders.arg("-r").arg("-g").arg(res)
    };
    let final_cmd = match cli.open_opt {
        cli::OpenOptEnum::Add => cmd.arg("-a"),
        cli::OpenOptEnum::New => cmd.arg("-n"),
        cli::OpenOptEnum::Reuse => cmd.arg("-r")
    };

    final_cmd.spawn().unwrap().wait_with_output().unwrap();
}

pub fn diff_on_editor(stdout: &mut ChildStdout, cli: &ParsedCli, original_file: &String) {
    let mut selected = String::new();
    stdout.read_to_string(&mut selected).expect("failed to read from stdout");

    if selected.is_empty()
    {
        return;
    }

    let res = helper::get_file_only(selected);

    let binding_code= &mut Command::new("code");
    let binding_insiders = &mut Command::new("code-insiders");
    let cmd = match cli.editor {
        cli::EditorEnum::Code => binding_code.arg("-d").arg(res).arg(original_file),
        cli::EditorEnum::Insiders => binding_insiders.arg("-d").arg(res).arg(original_file)
    };

    cmd.spawn().unwrap().wait_with_output().unwrap();
}