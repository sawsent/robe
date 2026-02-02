pub const HELP: &str = r#"robe — switch between named dotfile configurations

Usage:
  robe add <tool>/<profile> [-r <path>] [-f]
  robe use <tool>/<profile>
  robe list [tool]
  robe view <tool> | <tool>/<profile>
  robe rm <tool> | <tool>/<profile>

Commands:
  add      save current config as a profile
  use      activate a profile
  list     list tools or profiles
  view     display current config or a stored profile
  rm       remove a profile or all profiles of a tool

Options:
  -h, --help      show help
  -v, --version   show version
"#;

pub fn help_with_storage_and_config(storage_file: &str, config_file: &str) -> String {
    format!(
        "{}\nStorage:\n{}\n\nConfig:\n{}",
        HELP, storage_file, config_file
    )
}

pub const VERSION: &str = "robe version 0.1.0";
