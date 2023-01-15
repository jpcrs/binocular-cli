use std::{ffi::OsStr, process::{Command, Stdio}};

use crate::cli::{ParsedCli, self};

use super::{file_content::FileContent, file_name::FileName, folder_name::FolderName, open_editor};

const HELP: &str = "--bind=ctrl-h:preview:printf '\
'\"${YELLOW}shortcuts:\"'

'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+g${NORMAL}    ${NORMAL}-- Change to grep mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+f${NORMAL}    ${NORMAL}-- Change to file names mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+d${NORMAL}    ${NORMAL}-- Change to folders mode\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+n${NORMAL}    ${NORMAL}-- Open the file/folder in a new editor window\"'
''
'\"${YELLOW}Preview Shortcuts:\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}shift+up${NORMAL}    ${NORMAL}-- Scroll preview up\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}shift+down${NORMAL}    ${NORMAL}-- Scroll preview down\"'
''
'\"${YELLOW}${BOLD}∆${NORMAL} ${GREEN}CTRL+p${NORMAL}    ${NORMAL}-- Toggle preview\"'
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

pub fn exec_fzf(file_content: &mut FileContent, file_name: &mut FileName, folder_name: &mut FolderName, cli: &ParsedCli) {
    let (cmd, prompt, preview) = match cli.mode {
        cli::ModeEnum::Grep => file_content.get_command_info(),
        cli::ModeEnum::File => file_name.get_command_info(),
        cli::ModeEnum::Directory => folder_name.get_command_info()
    };

    let new_window_cmd = match cli.editor {
        cli::EditorEnum::Code => open_editor::VSCODE_NEW_WINDOW,
        cli::EditorEnum::Insiders => open_editor::INSIDERS_NEW_WINDOW
    };

    let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");
    
    let mut args: Vec<&OsStr> = FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();
    
    let prompt_line = format!("--prompt={}", prompt);
    let open_editor_new_line = format!("--bind=ctrl-n:execute-silent({})+abort", new_window_cmd);
    let grep_line = format!("--bind=ctrl-g:reload({} {{q}})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-r)+rebind(change,ctrl-f)+rebind(change,ctrl-d)", file_content.parse_command(), file_content.prompt, file_content.preview);
    let file_line = format!("--bind=ctrl-f:reload({})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-f)+rebind(change,ctrl-r)+rebind(change,ctrl-d)", file_name.parse_command(), file_name.prompt, file_name.preview);
    let directory_line = format!("--bind=ctrl-d:reload({})+change-prompt({})+change-preview-window(hidden)+change-preview({})+unbind(change,ctrl-d)+rebind(change,ctrl-r)+rebind(change,ctrl-f)", folder_name.parse_command(), folder_name.prompt, folder_name.preview);
    let preview_line = format!("--preview={}", preview);
    let preview_size = format!("--preview-window=50%,+{{2}}+3/3,~3");
    let query_line = format!("-q {}", &cli.query);

    args.push(OsStr::new(&prompt_line));
    args.push(OsStr::new(&open_editor_new_line));
    args.push(OsStr::new(&grep_line));
    args.push(OsStr::new(&file_line));
    args.push(OsStr::new(&directory_line));
    args.push(OsStr::new(&preview_line));
    args.push(OsStr::new(&preview_size));
    args.push(OsStr::new(&query_line));

    let mut fzf = Command::new("fzf")
        .args(args)
        .stdin(std_out)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = fzf.stdout.as_mut().expect("failed to open stdout");
    open_editor::open_on_editor(stdout, cli);
}