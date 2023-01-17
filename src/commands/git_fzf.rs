use std::{ffi::OsStr, process::{Command, Stdio}, io::{BufReader, BufRead, Write}};

use crate::{cli::{ParsedCli, self}, helper};

use super::{file_content::FileContent, file_name::FileName, folder_name::FolderName, open_editor, git_folders::GitFolders};

const HELP: &str = "--bind=ctrl-h:preview:printf '\
'\"shortcuts:\"'

'\"∆ CTRL+g    -- Change to grep mode\"'
''
'\"∆ CTRL+f    -- Change to file names mode\"'
''
'\"∆ CTRL+d    -- Change to folders mode\"'
''
'\"∆ CTRL+n    -- Open the file/folder in a new editor window\"'
''
'\"Preview Shortcuts:\"'
''
'\"∆ shift+up    -- Scroll preview up\"'
''
'\"∆ shift+down    -- Scroll preview down\"'
''
'\"∆ CTRL+p    -- Toggle preview\"'
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
    "--bind=shift-up:preview-up,shift-down:preview-down",
    "--bind=ctrl-p:toggle-preview",
    HELP,
    "--bind=ctrl-x:change-preview-window(80%,border-bottom|50%,border-bottom|20%,border-bottom|hidden|)",
    "--no-height",
    "--delimiter=:"
];

pub fn exec_git_fzf(file_content: &mut FileContent, file_name: &mut FileName, folder_name: &mut FolderName, git_folders: &mut GitFolders, cli: &ParsedCli) {
    let (cmd, prompt, preview) = match cli.mode {
        cli::ModeEnum::Grep => file_content.get_command_info(),
        cli::ModeEnum::File => file_name.get_command_info(),
        cli::ModeEnum::Directory => folder_name.get_command_info(),
        cli::ModeEnum::Projects => git_folders.get_command_info(),
    };

    let new_window_cmd = match cli.editor {
        cli::EditorEnum::Code => open_editor::VSCODE_NEW_WINDOW,
        cli::EditorEnum::Insiders => open_editor::INSIDERS_NEW_WINDOW
    };

    let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");
    let reader = BufReader::new(std_out);

    println!("{}", helper::get_command_string(cmd));

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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let modified_line = line.trim_end_matches(".git/");
        fzf_stdin.write_all(modified_line.as_bytes()).unwrap();
        fzf_stdin.write_all(b"\n").unwrap();
    }
    drop(fzf_stdin);

    let stdout = fzf.stdout.as_mut().expect("failed to open stdout");
    open_editor::open_on_editor(stdout, cli);
}