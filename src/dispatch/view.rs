use crate::domain::View;
use crate::errors::RobeError;
use crate::registry::Registry;
use std::fs;
use std::path::Path;

pub fn view(cmd: &View, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;

    let fp = match &cmd.profile {
        Some(profile) => {
            target_registry.assert_profile_exists(profile)?;
            registry
                .base_path
                .to_path_buf()
                .join(&cmd.target)
                .join(profile)
        }
        None => target_registry.real_path,
    };

    let formatted = if fp.is_dir() {
        format_dir(&fp, cmd.raw)?
    } else {
        format_file(&fp, cmd.raw)?
    };

    println!("{}", formatted);

    Ok(())
}

fn format_file(fp: &Path, raw: bool) -> Result<String, RobeError> {
    let mut out = "".to_string();
    if !raw {
        out.push_str(&format!("File: {}\n", fp.display()));
        out.push_str("------------------------------\n\n");
    }
    out.push_str(&fs::read_to_string(fp)?.to_string());
    if !raw {
        out.push_str("\n------------------------------\n");
        out.push_str(&format!("Path: {}", fp.display()));
    }
    Ok(out)
}

fn format_dir(fp: &Path, raw: bool) -> Result<String, RobeError> {
    let mut out = "".to_string();
    if !raw {
        out.push_str(&format!("Directory: {}\n\n", fp.display()));
    }
    out.push_str(&format_dir_raw(fp)?);
    if !raw {
        out.push_str(&format!("\nPath: {}", fp.display()));
    }
    Ok(out)
}

fn format_dir_raw(fp: &Path) -> Result<String, RobeError> {
    let mut out = "".to_string();

    let entries: Vec<_> = fs::read_dir(fp)?.collect::<Result<Vec<_>, _>>()?;
    let mut dir_entries: Vec<_> = entries.iter().filter(|p| p.path().is_dir()).collect();
    dir_entries.sort_by_key(|e| e.file_name());
    let mut other_entries: Vec<_> = entries.iter().filter(|p| !p.path().is_dir()).collect();
    other_entries.sort_by_key(|e| e.file_name());

    for entry in dir_entries {
        let f = entry.file_name();
        let fname = f.to_string_lossy();
        out.push_str(&format!("{}/\n", fname));
    }
    for entry in other_entries {
        let f = entry.file_name();
        let fname = f.to_string_lossy();
        out.push_str(&format!("{}\n", fname));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_format_file_pretty() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.txt");

        fs::write(&file, "hello world").unwrap();

        let output = format_file(&file, false).unwrap();

        assert!(output.contains("File:"));
        assert!(output.contains("hello world"));
        assert!(output.contains("Path:"));
    }

    #[test]
    fn test_format_file_raw() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.txt");

        fs::write(&file, "hello").unwrap();

        let output = format_file(&file, true).unwrap();

        assert_eq!(output, "hello");
    }

    #[test]
    fn test_format_dir_raw_orders_dirs_first() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir(base.join("b_dir")).unwrap();
        fs::create_dir(base.join("a_dir")).unwrap();
        fs::write(base.join("z.txt"), "z").unwrap();
        fs::write(base.join("a.txt"), "a").unwrap();

        let output = format_dir_raw(base).unwrap();

        let expected = "\
a_dir/
b_dir/
a.txt
z.txt
";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_dir_pretty() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir(base.join("folder")).unwrap();
        fs::write(base.join("file.txt"), "x").unwrap();

        let output = format_dir(base, false).unwrap();

        assert!(output.contains("Directory:"));
        assert!(output.contains("folder/"));
        assert!(output.contains("file.txt"));
        assert!(output.contains("Path:"));
    }

    #[test]
    fn test_format_dir_raw_flag() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir(base.join("folder")).unwrap();

        let output = format_dir(base, true).unwrap();

        assert_eq!(output, "folder/\n");
    }

    #[test]
    fn test_view_file_profile() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let wardrobe = dir.path().join("wardrobe");
        let target_dir = wardrobe.join("tmux");
        fs::create_dir_all(&target_dir).unwrap();

        let profile_file = target_dir.join("work");
        fs::write(&profile_file, "set -g mouse on").unwrap();

        let mut registry = Registry {
            base_path: wardrobe.clone(),
            targets: std::collections::HashMap::new(),
        };

        registry.targets.insert(
            "tmux".to_string(),
            crate::registry::TargetRegistry {
                name: "tmux".to_string(),
                real_path: profile_file.clone(),
                profiles: vec!["work".to_string()],
            },
        );

        let cmd = View {
            target: "tmux".to_string(),
            profile: Some("work".to_string()),
            raw: false,
        };

        let result = view(&cmd, &registry);

        assert!(result.is_ok());
    }

    #[test]
    fn test_view_directory_profile() {
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let wardrobe = dir.path().join("wardrobe");
        let target_dir = wardrobe.join("nvim");
        fs::create_dir_all(&target_dir).unwrap();

        let profile_dir = target_dir.join("minimal");
        fs::create_dir_all(&profile_dir).unwrap();

        fs::write(profile_dir.join("init.lua"), "print('hi')").unwrap();
        fs::create_dir(profile_dir.join("lua")).unwrap();

        let mut registry = Registry {
            base_path: wardrobe.clone(),
            targets: std::collections::HashMap::new(),
        };

        registry.targets.insert(
            "nvim".to_string(),
            crate::registry::TargetRegistry {
                name: "nvim".to_string(),
                real_path: profile_dir.clone(),
                profiles: vec!["minimal".to_string()],
            },
        );

        let cmd = View {
            target: "nvim".to_string(),
            profile: Some("minimal".to_string()),
            raw: false,
        };

        let result = view(&cmd, &registry);

        assert!(result.is_ok());
    }
}
