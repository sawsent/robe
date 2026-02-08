pub const HELP: &str = r#"robe — switch between named dotfile configurations

Usage:
  robe add <target>/<profile> [-r <path>] [-f]
  robe use <target>/<profile>
  robe view <target>[/profile]
  robe edit <target>[/profile]
  robe list [target]
  robe rm <target>/<profile>
  robe rm <target>

Commands:
  add       save current config as a profile
            -r, --register <path>  register file or directory to manage
            -f, --force            overwrite existing profile or registration

  use       activate a profile

  view      print config contents
            file → prints contents
            dir  → lists entries

  edit      open config in $EDITOR (defaults to vi)

  list      list targets or profiles

  rm        remove a stored profile or all profiles of a target

Options:
  -h, --help       show help
  -v, --version    show version
"#;

pub fn help_with_storage_and_config(storage_file: &str, config_file: &str) -> String {
    format!(
        "{}\nStorage:\n{}\n\nConfig:\n{}",
        HELP, storage_file, config_file
    )
}

pub const VERSION: &str = "robe version 0.0.6";
