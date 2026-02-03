pub const HELP: &str = r#"robe — switch between named dotfile configurations

Usage:
  robe add <target>/<profile> [-r <path>] [-f]
  robe use <target>/<profile>
  robe list [target]
  robe view <target> | <target>/<profile>
  robe rm <target> | <target>/<profile>

Commands:
  add      save current config as a profile
  use      activate a profile
  list     list targets or profiles
  view     display current config or a stored profile
  rm       remove a profile or all profiles of a target

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
