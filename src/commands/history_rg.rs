use std::{process::{Command, Stdio}, path::PathBuf};

#[allow(dead_code)]
pub struct HistoryRg {
    pub command: Command,
    pub file: String,
}


#[allow(dead_code)]
impl HistoryRg {
    pub fn new(path: &PathBuf, file: &PathBuf) -> Self {
        let formated_file = file.to_str().map(|s| s.replace(" ", "%20")).unwrap_or(String::new());

        let mut cmd = Command::new("rg");
        cmd.arg("-l").arg("-g").arg("entries.json").arg(formated_file).arg(path).stdout(Stdio::piped());
        HistoryRg {
            command: cmd,
            file: String::from(file.to_str().unwrap()),
        }
    }
}