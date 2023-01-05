use clap::{Args, Parser};
use std::{path::PathBuf, env};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Code
    #[clap(flatten)]
    editor: Editor,

    /// Mode
    #[clap(flatten)]
    mode: Mode,

    /// Query
    #[arg(short, long)]
    query: Option<String>,

    /// Path
    #[arg(short, long)]
    paths: Option<Vec<PathBuf>>,

    /// ignore line
    #[arg(long, default_value = "false")]
    ignore_line: bool,

    /// Code
    #[clap(flatten)]
    shortcut_editor: ShortcutEditor,
}

#[derive(Args, Debug)]
struct Editor {
    /// Open result on code
    #[arg(short, long, group = "editor")]
    code: bool,

    /// Open result on code-insiders
    #[arg(short, long, group = "editor")]
    insiders: bool,

    /// Open result on vim
    #[arg(short, long, group = "editor")]
    vim: bool,

    /// Open result on explorer
    #[arg(short, long, group = "editor")]
    explorer: bool,
}

impl Editor {
    pub fn parse_to_enum(&self) -> EditorEnum {
        if self.code {
            return EditorEnum::Code;
        } else if self.insiders {
            return EditorEnum::Insiders;
        } else if self.vim {
            return EditorEnum::Vim;
        } else if self.explorer {
            return EditorEnum::Explorer;
        } else {
            return EditorEnum::Echo;
        }
    }
}

#[derive(Debug)]
pub enum EditorEnum {
    Code,
    Insiders,
    Vim,
    Explorer,
    Echo,
}

#[derive(Args, Debug)]
struct ShortcutEditor {
    /// Shortcut opens on code
    #[arg(long, group = "shortcuteditor")]
    shortcut_code: bool,

    /// Shortcut opens on code-insiders
    #[arg(long, group = "shortcuteditor")]
    shortcut_insiders: bool,

    /// Shortcut opens on vim
    #[arg(long, group = "shortcuteditor")]
    shortcut_vim: bool,

    /// Shortcut opens on explorer
    #[arg(long, group = "shortcuteditor")]
    shortcut_explorer: bool,
}

impl ShortcutEditor {
    pub fn parse_to_enum(&self) -> EditorEnum {
        if self.shortcut_code {
            return EditorEnum::Code;
        } else if self.shortcut_insiders {
            return EditorEnum::Insiders;
        } else if self.shortcut_vim {
            return EditorEnum::Vim;
        } else if self.shortcut_explorer {
            return EditorEnum::Vim;
        } else {
            return EditorEnum::Echo;
        }
    }
}

#[derive(Args, Debug)]
struct Mode {
    /// Grep
    #[arg(short, long, group = "mode")]
    grep: bool,

    /// File
    #[arg(short, long, group = "mode")]
    file: bool,

    /// Directory
    #[arg(short, long, group = "mode")]
    directory: bool,
}

impl Mode {
    pub fn parse_to_enum(&self) -> ModeEnum {
        if self.file {
            return ModeEnum::File;
        } else if self.directory {
            return ModeEnum::Directory;
        } else {
            return ModeEnum::Grep;
        }
    }
}

impl Cli {
    pub fn build_paths(&self) -> Vec<PathBuf> {
        let curr_path = env::current_dir().unwrap();
        let mut final_paths: Vec<PathBuf> = vec![];
        if let Some(paths) = &self.paths {
            for path in paths {
                final_paths.push(path.to_path_buf());
            }
            return final_paths;
        }
        final_paths.push(curr_path);
        return final_paths;
    }
    
    pub fn build_query(&self) -> String {
        if let Some(query) = &self.query {
            return String::from(query);
        }
        return String::from("a");
    }
}

#[derive(Debug)]
pub enum ModeEnum {
    Grep,
    File,
    Directory,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedCli {
    pub editor: EditorEnum,
    pub shortcut_editor: EditorEnum,
    pub path: Vec<PathBuf>,
    pub query: String,
    pub ignore_line: bool,
    pub mode: ModeEnum
}

impl ParsedCli {
    pub fn new(cli: &Cli) -> ParsedCli {
        ParsedCli {
            editor: cli.editor.parse_to_enum(),
            shortcut_editor: cli.shortcut_editor.parse_to_enum(),
            path: cli.build_paths(),
            query: cli.build_query(),
            ignore_line: cli.ignore_line,
            mode: cli.mode.parse_to_enum()
        }
    }
}