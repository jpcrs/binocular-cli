use clap::Parser;
use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Stdio},
};

#[path = "consts.rs"]
mod consts;

pub struct Binocular {
    pub grep_command: Command,
    pub file_command: Command,
    pub folder_command: Command,
    // pub fzf_command: Command
}

impl Binocular {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        Binocular {
            grep_command: Self::init_command(paths, "rg", consts::RG_PARAMS),
            file_command: Self::init_command(paths, "fd", consts::FILES_PARAMS),
            folder_command: Self::init_command(paths, "fd", consts::FOLDERS_PARAMS)
        }
    }

    pub fn parse_grep_command(&self) -> String {
        return Self::get_command_string(&self.grep_command);
    }

    pub fn parse_file_command(&self) -> String {
        return Self::get_command_string(&self.file_command);
    }

    pub fn parse_folder_command(&self) -> String {
        return Self::get_command_string(&self.folder_command);
    }

    fn init_command(paths: &Vec<PathBuf>, name: &str, params: &'static [&str]) -> Command {
        let curr_path = env::current_dir().unwrap();
        let mut args: Vec<&OsStr> = params.iter().map(|f| OsStr::new(f)).collect();

        for path in paths {
            args.push(OsStr::new(path.as_os_str()));
        }

        let mut cmd = Command::new(name);
        cmd.args(args).stdout(Stdio::piped());
        return cmd;
    }

    fn fzf_command(mut cmd: Command, query: &Option<String>) -> Command {
        let std_out = cmd.spawn().unwrap().stdout.expect("Failed to get the command stdout");

        let mut args: Vec<&OsStr> = consts::FZF_PARAMS.iter().map(|f| OsStr::new(f)).collect();

        args.push(OsStr::new("--prompt=updatemeeeeeeeee"));
        args.push(OsStr::new("--bind=ctrl-o:execute-silent($$$COMMAND-TO-OPEN-VSCODE$$$)+abort"));
        args.push(OsStr::new("--bind=ctrl-n:execute-silent($$$COMMAND-TO-OPEN-VSCODE-NEW-WINDOW$$$)+abort"));
        args.push(OsStr::new("--bind=ctrl-g:reload($GREP_CMD {q})+change-prompt($GREP_PROMPT)+change-preview-window(50%)+change-preview($GREP_PREVIEW_STYLE)+unbind(change,ctrl-r)+rebind(change,ctrl-f)+rebind(change,ctrl-d)"));
        args.push(OsStr::new("--bind=ctrl-f:reload($FILE_CMD)+change-prompt($FILE_PROMPT)+change-preview-window(50%)+change-preview($FILE_PREVIEW_STYLE)+unbind(change,ctrl-f)+rebind(change,ctrl-r)+rebind(change,ctrl-d)"));
        args.push(OsStr::new("--bind=ctrl-d:reload($DIRECTORY_CMD)+change-prompt($DIRECTORY_PROMPT)+change-preview-window(hidden)+change-preview($DIRECTORY_PREVIEW_STYLE)+unbind(change,ctrl-d)+rebind(change,ctrl-r)+rebind(change,ctrl-f)"));
        args.push(OsStr::new("--preview=bat --color=always {1} --highlight-line {2}"));
        
        if let Some(q) = query {
            args.push(OsStr::new("-q"));
            args.push(OsStr::new(q));
        }

        let mut fzf = Command::new("fzf");
        fzf.args(args).stdin(std_out);
        return fzf;
    }

    fn get_command_string(cmd: &Command) -> String {
        let mut s = String::new();
        for arg in cmd.get_args().into_iter() {
            if let Some(str) = arg.to_str() {
                s.push_str(str);
                s.push_str(" ");
            }
        }
        let res = cmd.get_program().to_str().unwrap();
        let mut res2 = res.to_string();
        res2.push_str(" ");
        res2.push_str(&s);
        return res2;
    }
}