use std::{process::{ChildStdout, Command}, io::Read};

use crate::{helper, cli::{ParsedCli, self}};

pub const VSCODE_NEW_WINDOW: &str ="code -g -n {1}:{2}";
pub const INSIDERS_NEW_WINDOW: &str ="code-insiders -g -n {1}:{2}";

pub fn open_on_editor(stdout: &mut ChildStdout, cli: &ParsedCli) {
    let mut selected = String::new();
    stdout.read_to_string(&mut selected).expect("failed to read from stdout");

    let res = helper::get_file_and_line(selected);


    let binding_code= &mut Command::new("code");
    let binding_insiders = &mut Command::new("code");
    let cmd = match cli.editor {
        cli::EditorEnum::Code => binding_code.arg("-g").arg(res),
        cli::EditorEnum::Insiders => binding_insiders.arg("-g").arg(res)
    };
    cmd.spawn().unwrap().wait_with_output().unwrap();
}