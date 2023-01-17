use std::{path::PathBuf, ffi::OsStr, process::{Command, Stdio}};

pub fn get_file_and_line(path: String) -> String {
    let first_colon_index = path.find(':').map_or(path.len(), |i| i);
    if first_colon_index >= path.len() {
        return format!("{}", &path[..path.len()-1]);
    }
    let second_colon_index = path[first_colon_index+1..].find(':').map_or(path.len(), |i| i+first_colon_index+1);

    let file_name = &path[..first_colon_index];
    let line_number = &path[first_colon_index+1..second_colon_index];
    if line_number.is_empty() {
        format!("{}:2", file_name)
    } else {
        format!("{}:{}", file_name, line_number)
    }
}

pub fn get_file_only(path: String) -> String {
    let first_colon_index = path.find(':').map_or(path.len(), |i| i);
    if first_colon_index >= path.len() {
        return format!("{}", &path[..path.len()-1]);
    }
    return format!("{}", &path[..first_colon_index]);
}

pub fn init_command(paths: &Vec<PathBuf>, name: &str, params: &'static [&str]) -> Command {
    let mut args: Vec<&OsStr> = params.iter().map(|f| OsStr::new(f)).collect();

    for path in paths {
        args.push(OsStr::new(path.as_os_str()));
    }

    let mut cmd = Command::new(name);
    cmd.args(args).stdout(Stdio::piped());
    return cmd;
}

pub fn get_command_string(cmd: &Command) -> String {
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