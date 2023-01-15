use std::{process::{Command, Stdio}, ffi::OsStr, io::{BufReader, BufRead}, path::PathBuf};

use crate::{cli::{self, ParsedCli}, file_content::FileContent, history_rg::HistoryRg, open_editor};

const HELP: &str = "--bind=ctrl-h:preview:printf '\
'\"${YELLOW}shortcuts:\"'

'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+g${NORMAL}    ${NORMAL}-- Change to grep mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+f${NORMAL}    ${NORMAL}-- Change to file names mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+d${NORMAL}    ${NORMAL}-- Change to folders mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+o${NORMAL}    ${NORMAL}-- Open the file/folder in the defined editor.\"'
              '\"${YELLOW}default: ${NORMAL}line=\\{2}; code -g \\{1}\\${line:-:\\{2}}\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+n${NORMAL}    ${NORMAL}-- Open the file/folder in a new editor window\"'
              '\"${YELLOW}default: ${NORMAL}line=\\{2}; code -g -n \\{1}\\${line:-:\\{2}}\"'
'";

const FZF_PARAMS: &'static [&str] = &[
    "--exact",
    "--ansi",
    "--multi",
    "--border",
    "--info=hidden",
    "--no-separator",
    "--layout=reverse",
    "--header= / CTRL-H (HELP!) /",
    "--color=hl:-1:underline,hl+:-1:underline:reverse",
    "--bind=change:top",
    "--bind=shift-up:preview-page-up,shift-down:preview-page-down",
    "--bind=ctrl-p:toggle-preview",
    HELP,
    "--bind=ctrl-x:change-preview-window(80%,border-bottom|50%,border-bottom|20%,border-bottom|hidden|)",
    "--no-height",
    "--delimiter=:"
];


pub fn history_fzf_commmand(file_content: &mut FileContent, history_rg: &mut HistoryRg, cli: &ParsedCli) {
    let new_window_cmd = match cli.editor {
        cli::EditorEnum::Code => open_editor::VSCODE_NEW_WINDOW,
        cli::EditorEnum::Insiders => open_editor::INSIDERS_NEW_WINDOW
    };

    let rg_output = history_rg.command.output().unwrap();
    if rg_output.status.success() {
        let reader = BufReader::new(rg_output.stdout.as_slice());
        let first_line = reader.lines().next().map(|l| l.unwrap()).unwrap_or(String::new());
        let pathbuf = PathBuf::from(first_line);
        let folder = pathbuf.parent().unwrap().to_string_lossy().to_string();

        file_content.command.arg(folder);

        let std_out = file_content.command.spawn().unwrap().stdout.expect("Failed to get the command stdout");

        let mut args: Vec<&OsStr> = FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();
        
        let prompt_line = format!("--prompt={}", "Grep > ");
        let open_editor_new_line = format!("--bind=ctrl-n:execute-silent({})+abort", new_window_cmd);
        let preview_line = format!("--preview=delta {{1}} {} --diff-highlight --line-numbers --side-by-side -w ${{FZF_PREVIEW_COLUMNS:-$COLUMNS}}", history_rg.file);
        let preview_size = format!("--preview-window=90%,+{{2}}+3/3,~3,down");
        let query_line = format!("-q {}", &cli.query);

        args.push(OsStr::new(&prompt_line));
        args.push(OsStr::new(&open_editor_new_line));
        args.push(OsStr::new(&preview_line));
        args.push(OsStr::new(&preview_size));
        args.push(OsStr::new(&query_line));

        let mut fzf = Command::new("fzf").args(args).stdin(std_out).stdout(Stdio::piped()).spawn().unwrap();

        let stdout = fzf.stdout.as_mut().expect("failed to open stdout");

        open_editor::open_on_editor(stdout, cli);
    } else {
        eprintln!("Command failed with status: {}", rg_output.status);
    }
}