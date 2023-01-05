pub const VSCODE_EDITOR_COMMAND: &str ="code -g {1}:{2}";
pub const VSCODE_EDITOR_COMMAND_NEW_WINDOW: &str ="code -g -n {1}:{2}";

pub const INSIDERS_EDITOR_COMMAND: &str ="line={2}; code-insiders -g {1}:{2}";
pub const INSIDERS_EDITOR_COMMAND_NEW_WINDOW: &str ="line={2}; code-insiders -g -n {1}:{2}";

//binocular -c -p \\\"\"$(echo $(rg -l -g 'entries.json' $(echo ${file} | sed -e 's/ /%20/g') '/Users/jpcrs/Library/Application Support/Code - Insiders/User/History/') | sed 's:[^/]*$::')\"\\\"
/*
1. Get File, change spaces to %20, using: 
    $(echo ${file} | sed -e 's/ /%20/g')
2. Get the result, use ripgrep to find a file that contains this $file in the history folder. using: 
    rg -l -g 'entries.json' $(echo ${file} | sed -e 's/ /%20/g') '/Users/jpcrs/Library/Application Support/Code - Insiders/User/History/'
3. Get the result, remove the last part after "/", it's the folder that we have to execute ripgrep.
    echo $(rg -l -g 'entries.json' $(echo ${file} | sed -e 's/ /%20/g') '/Users/jpcrs/Library/Application Support/Code - Insiders/User/History/') | sed -e 's:[^/]*$::'
 */

pub const FILES_PREVIEW: &str = "bat --color=always {}";
pub const FILES_PROMPT: &str = "Files > ";
pub const FILES_PARAMS: &'static [&str] = &[
    "-tf",
    "-HI", 
    "--exclude",
    ".git",
    "--exclude",
    "node_modules",
    ""
];

pub const FOLDER_PREVIEW: &str = "ls -al --color=always {}";
pub const FOLDER_PROMPT: &str = "Folder > ";
pub const FOLDERS_PARAMS: &'static [&str] = &[
    "-td",
    "-HI", 
    "--exclude",
    ".git",
    "--exclude",
    "node_modules",
    ""
];

pub const RG_PREVIEW: &str = "bat --color=always {1} --highlight-line {2}";
pub const RG_PROMPT: &str = "Grep > ";
pub const RG_PARAMS: &'static [&str] = &[
    "--hidden",
    "--column",
    "--line-number",
    "--no-heading",
    "--color=always",
    "--smart-case",
    ""
];

pub const HELP: &str = "--bind=ctrl-h:preview:printf '\
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
    HELP,
    "--bind=ctrl-x:change-preview-window(80%,border-bottom|50%,border-bottom|20%,border-bottom|hidden|)",
    "--preview-window=50%,+{2}+3/3,~3",
    "--no-height",
    "--delimiter=:"
];