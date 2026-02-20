use crate::domain::Edit;
use crate::errors::RobeError;
use crate::registry::Registry;
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn edit_with_runner<F>(
    cmd: &Edit,
    registry: &Registry,
    runner: F,
) -> Result<(), RobeError> 
where F: Fn(&str, &str) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;

    let fp: PathBuf = match &cmd.profile {
        Some(profile) => {
            target_registry.assert_profile_exists(profile)?;
            registry.base_path.join(&cmd.target).join(profile)
        }
        None => target_registry.real_path.clone(),
    };

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".into());

    runner(&editor, &fp.to_string_lossy().to_string())?;

    Ok(())
}

pub fn edit(cmd: &Edit, registry: &Registry) -> Result<(), RobeError> {
    edit_with_runner(cmd, registry, |editor, path| {
        let args = vec![path];
        Command::new(editor).args(args).status()?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use crate::registry::{Registry, TargetRegistry};
    use crate::domain::Edit;

    #[test]
    fn test_edit_with_mock_runner() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myconfig.txt");
        fs::write(&file_path, "hello world").unwrap();

        let mut registry = Registry::default();
        registry.base_path = dir.path().to_path_buf();
        registry.targets.insert(
            "myconfig".into(),
            TargetRegistry {
                name: "myconfig".into(),
                real_path: file_path.clone(),
                profiles: vec!["default".into()],
            },
        );

        let cmd = Edit {
            target: "myconfig".into(),
            profile: None,
        };

        unsafe {
            env::set_var("EDITOR", "true");
        }

        let result = edit_with_runner(&cmd, &registry, |editor: &str, path: &str| {
            assert_eq!(editor, "true");
            assert_eq!(path, file_path.to_str().unwrap());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_profile_with_mock_runner() {
        let wardrobe_dir = tempdir().unwrap();

        let dir = tempdir().unwrap();
        let real_file_path = dir.path().join("myconfig.txt");
        fs::write(&real_file_path, "hello world").unwrap();

        let mut registry = Registry::default();
        registry.base_path = wardrobe_dir.path().to_path_buf();
        registry.targets.insert(
            "myconfig".into(),
            TargetRegistry {
                name: "myconfig".into(),
                real_path: real_file_path.clone(),
                profiles: vec!["default".into()],
            },
        );

        let cmd = Edit {
            target: "myconfig".into(),
            profile: Some("default".into()),
        };

        unsafe {
            env::set_var("EDITOR", "true");
        }

        let result = edit_with_runner(&cmd, &registry, |editor: &str, path: &str| {
            assert_eq!(editor, "true");
            assert_eq!(path, wardrobe_dir.path().join("myconfig").join("default").to_str().unwrap());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myconfig.txt");
        fs::write(&file_path, "hello world").unwrap();

        let mut registry = Registry {
            base_path: dir.path().to_path_buf(),
            targets: std::collections::HashMap::new(),
        };
        registry.targets.insert(
            "myconfig".into(),
            TargetRegistry {
                name: "myconfig".into(),
                real_path: file_path.clone(),
                profiles: vec![],
            },
        );

        unsafe {
            env::set_var("EDITOR", "true");
        }

        let cmd = Edit {
            target: "myconfig".into(),
            profile: None,
        };

        let result = super::edit(&cmd, &registry);
        assert!(result.is_ok());
    }
}
