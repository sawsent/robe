pub const HELP: &str = r#"suit — switch between named dotfile configurations

Usage:
  suit add <tool>/<profile> [-r <path>] [-f]
  suit use <tool>/<profile>
  suit list [tool]
  suit current <tool>
  suit rename <tool> <new_tool> [-f]
  suit restore <tool>
  suit rm <tool>/<profile> [-f]
  suit rm <tool> [-f]

Commands:
  add       save current config as a profile
            -r, --register <path>  register file if default missing
  use       activate a profile
  list      list tools or profiles
  current   show active profile
  rename    rename a tool
  restore   restore a tool to its last pre-suit-load state
  rm        remove a stored profile or all profiles of a tool

Options:
  -d, --dir <path>   storage directory (default: ~/.config/suit)
  -f, --force        overwrite, rename, or remove without prompting
  -h, --help         show help
  -v, --version      show version

Storage:
  <dir>/<tool>/<profile>

Notes:
  single-file configs only
  profiles are plain files
  no templating or automation

"#;

