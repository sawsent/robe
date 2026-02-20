pub const HELP: &str = r#"robe — switch between named dotfile configurations

Usage:
  robe add <target>/<profile> [-r <path>] [-f]
  robe use <target>/<profile>
  robe view <target>[/profile]
  robe edit <target>[/profile]
  robe list [target]
  robe ls [target]
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
  ls        alias list  

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

pub const VERSION: &str = "robe version 0.0.9";

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_help_with_storage_and_config() {
        let s_file = "storage/file/path";
        let c_file = "config/file/path";

        let result = help_with_storage_and_config(s_file, c_file);

        assert!(result.contains(HELP));
        assert!(result.contains(c_file));
        assert!(result.contains(s_file));
    }

    #[test]
    fn test_version() {
        if let Ok(c_version_string) = std::fs::read_to_string(PathBuf::from("Cargo.toml")) {
            let version_vec: Vec<&str> = VERSION.split(" ").collect();
            let version = version_vec[2];

            if !c_version_string.contains(version) {
                panic!("VERSION in help.rs not in line with Cargo.toml");
            }
        } else {
            panic!("Unable to test VERSION")
        }
    }
}
