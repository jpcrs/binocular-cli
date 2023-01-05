use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Stdio},
};

#[path = "consts.rs"]
mod consts;

#[derive(Debug)]
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

    fn get_command_string(cmd: &Command) -> String {
        let mut s = String::new();
        for arg in cmd.get_args().into_iter() {
            if let Some(str) = arg.to_str() {
                if str.is_empty() {
                    s.push_str("''");
                }
                else {
                    s.push_str(str);
                }
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