use std::{ffi::OsStr, process::{Command, Stdio}};

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
    //info
    "--exact",
    "--ansi",
    "--delimiter=:",
    // search
    "--border=sharp",
    "--border-label-pos=4,bottom",
    "--info=inline",
    "--no-separator",
    "--header=",
    "--margin=2,2,2,2",
    "--scrollbar=",
    //preview
    "--preview-window=border-sharp",
    "--preview-label=  Preview  ",
    //bindings
    "--bind=shift-up:preview-up,shift-down:preview-down",
    "--bind=ctrl-p:toggle-preview",
    HELP,
    "--bind=ctrl-x:change-preview-window(80%,border-sharp|hidden|20%,border-sharp|50%,border-sharp|)"
];

pub fn exec_fzf(file_content: &mut FileContent, file_name: &mut FileName, folder_name: &mut FolderName, git_folders: &mut GitFolders, cli: &ParsedCli) {
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

    let append_cmd = match cli.editor {
        cli::EditorEnum::Code => open_editor::VSCODE_APPEND_WINDOW,
        cli::EditorEnum::Insiders => open_editor::INSIDERS_APPEND_WINDOW
    };

    let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");
    
    let mut args: Vec<&OsStr> = FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();
    
    let prompt_line = format!("--border-label={}", prompt);
    let open_editor_new_line = format!("--bind=ctrl-n:execute-silent({})+abort", new_window_cmd);
    let append_editor_line = format!("--bind=ctrl-a:execute-silent({})+abort", append_cmd);
    let grep_line = format!("--bind=ctrl-g:reload({} {{q}})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-r)+rebind(change,ctrl-f)+rebind(change,ctrl-d)", file_content.parse_command(), file_content.prompt, file_content.preview);
    let file_line = format!("--bind=ctrl-f:reload({})+change-prompt({})+change-preview-window(50%)+change-preview({})+unbind(change,ctrl-f)+rebind(change,ctrl-r)+rebind(change,ctrl-d)", file_name.parse_command(), file_name.prompt, file_name.preview);
    let directory_line = format!("--bind=ctrl-d:reload({})+change-prompt({})+change-preview-window(hidden)+change-preview({})+unbind(change,ctrl-d)+rebind(change,ctrl-r)+rebind(change,ctrl-f)", folder_name.parse_command(), folder_name.prompt, folder_name.preview);
    let preview_line = format!("--preview={}", preview);
    let preview_size = format!("--preview-window=50%,+{{2}}+3/3,~3");
    let query_line = format!("-q {}", &cli.query);

    args.push(OsStr::new(&prompt_line));
    args.push(OsStr::new(&open_editor_new_line));
    args.push(OsStr::new(&append_editor_line));
    args.push(OsStr::new(&grep_line));
    args.push(OsStr::new(&file_line));
    args.push(OsStr::new(&directory_line));
    args.push(OsStr::new(&preview_line));
    args.push(OsStr::new(&preview_size));
    if !cli.query.is_empty() {
        args.push(OsStr::new(&query_line));
    }

    let mut fzf = Command::new("fzf")
        .args(args)
        .stdin(std_out)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = fzf.stdout.as_mut().expect("failed to open stdout");
    open_editor::open_on_editor(stdout, cli);
}