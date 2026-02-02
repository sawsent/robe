pub const HELP: &str = r#"suit — switch between named dotfile configurations

Usage:
  suit add <tool>/<profile> [-r <path>] [-f]
  suit use <tool>/<profile>
  suit list [tool]
  suit rm <tool>/<profile>
  suit rm <tool>

Commands:
  add       save current config as a profile
            -r, --register <path>  register target file (override with -f)
            -f, --force            update the tool/profile with the current config or update the registered file path.
  use       activate a profile
  list      list tools or profiles
  rm        remove a stored profile or all profiles of a tool

Options:
  -h, --help         show help
  -v, --version      show version
"#;

pub fn help_with_storage_and_config(storage_file: &str, config_file: &str) -> String {
    format!("{}\nStorage:\n{}\n\nConfig:\n{}", HELP, storage_file, config_file)
}

pub const VERSION: &str = "suit version 0.1.0";
