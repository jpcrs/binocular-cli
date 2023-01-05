pub const FILES_PARAMS: &'static [&str] = &[
    "-tf",
    "-HI", 
    "--exclude",
    ".git",
    "--exclude",
    "node_modules",
    ""
];

pub const FOLDERS_PARAMS: &'static [&str] = &[
    "-td",
    "-HI", 
    "--exclude",
    ".git",
    "--exclude",
    "node_modules",
    ""
];

pub const RG_PARAMS: &'static [&str] = &[
    "--hidden",
    "--column",
    "--line-number",
    "--no-heading",
    "--color=always",
    "--smart-case",
    ""
];

pub const FZF_PARAMS: &'static [&str] = &[
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
    "--bind=ctrl-h:preview:printf 'HELP :)'",
    "--bind=ctrl-x:change-preview-window(80%,border-bottom|50%,border-bottom|20%,border-bottom|hidden|)",
    "--preview-window=50%,+{2}+3/3,~3",
    "--no-height",
    "--delimiter=:"
];

const OPEN_VSCODE: &str = "code -g f:l";
const OPEN_VSCODE_NEW_WINDOW: &str = "code -g -n f:l";

const OPEN_VSCODE_INSIDERS: &str = "code-insiders -g f:l";
const OPEN_VSCODE_INSIDERS_NEW_WINDOW: &str = "code -g -n f:l";

const OPEN_VIM: &str = "vim f +l";
const OPEN_VIM_NEW_WINDOW: &str = "vim f +l";