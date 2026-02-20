use crate::dispatch::io;
use crate::domain::Add;
use crate::errors::RobeError;
use crate::registry::{Registry, TargetMetadata};
use std::fs;
use std::path::Path;

/// Add a profile to a target.
/// Requires force if profile already exists.
/// Requires register if the target does not exist.
///
/// Behaviour:
/// copy the current file/dir at `real_path` to `path/to/wardrobe/<target>/<profile>`
/// update the target registry if -r
pub fn add(cmd: &Add, registry: &Registry) -> Result<(), RobeError> {
    if let Some(target_registry) = registry.get_target_registry(&cmd.target) {
        if target_registry.profiles.contains(&cmd.profile.to_string()) && !cmd.force {
            return Err(RobeError::message(format!(
                "Profile {}/{} already exists. Use `-f` to update.",
                &cmd.target, &cmd.profile
            )));
        }

        let robe_target_path = Path::join(&registry.base_path, &cmd.target);

        let mut real_path = target_registry.real_path.clone();

        if let Some(register_path) = cmd.to_register.clone() {
            if !cmd.force {
                return Err(RobeError::message(format!(
                    "Target {} is already registered. Use `-f` to update registered target path.",
                    &cmd.target
                )));
            }
            real_path = register_path.clone();
            io::store_metadata(&robe_target_path, &TargetMetadata::create(&register_path)?)?;
        }

        let mut target_path = registry.base_path.clone();
        target_path.push(cmd.target.clone());
        target_path.push(cmd.profile.clone());

        io::replace_file_or_dir(&real_path, &target_path)?;
    } else if let Some(register_path) = cmd.to_register.clone() {
        let target_path = Path::join(&registry.base_path, &cmd.target);
        let mut target_path = target_path.clone();
        fs::create_dir_all(&target_path)?;

        let meta = TargetMetadata::create(&register_path)?;

        io::store_metadata(&target_path, &meta)?;

        target_path.push(cmd.profile.clone());

        io::replace_file_or_dir(&register_path, &target_path)?;
    } else {
        return Err(RobeError::message(format!(
            "Target {} not registered. Use -r <file> to register.",
            &cmd.target
        )));
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use crate::registry::{TargetRegistry, TargetMetadata, Registry};

    fn create_file(path: &std::path::Path, content: &str) {
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_add_new_target_with_register() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path();

        // Create a fake file to register
        let source_file = temp.path().join("config.txt");
        create_file(&source_file, "hello world");

        // Empty registry
        let registry = Registry {
            base_path: wardrobe.to_path_buf(),
            targets: std::collections::HashMap::new(),
        };

        // Command to add
        let cmd = Add {
            target: "tmux".to_string(),
            profile: "work".to_string(),
            to_register: Some(source_file.clone()),
            force: false,
        };

        // Call the function
        add(&cmd, &registry).unwrap();

        // Assert target folder exists
        let target_dir = wardrobe.join("tmux");
        assert!(target_dir.exists() && target_dir.is_dir());

        // Assert profile exists
        let profile_path = target_dir.join("work");
        assert!(profile_path.exists() && profile_path.is_file());

        // Assert content copied
        let content = fs::read_to_string(profile_path).unwrap();
        assert_eq!(content, "hello world");

        // Assert metadata file exists
        assert!(target_dir.join("meta.toml").exists());
    }

    #[test]
    fn test_add_existing_profile_requires_force() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path();

        // Registry with one existing profile
        let mut tr = TargetRegistry {
            name: "tmux".to_string(),
            real_path: temp.path().join("config.txt"),
            profiles: vec!["work".to_string()],
        };
        let mut targets = std::collections::HashMap::new();
        targets.insert("tmux".to_string(), tr.clone());

        let registry = Registry {
            base_path: wardrobe.to_path_buf(),
            targets,
        };

        let cmd = Add {
            target: "tmux".to_string(),
            profile: "work".to_string(),
            to_register: None,
            force: false,
        };

        // Should fail because profile exists and no -f
        let res = add(&cmd, &registry);
        assert!(res.is_err());
        assert!(format!("{}", res.unwrap_err())
            .contains("Profile tmux/work already exists"));
    }

    #[test]
    fn test_add_existing_profile_with_force() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path().join("wardrobe");

        // Create a file to copy
        let source_file = temp.path().join("config.txt");
        create_file(&source_file, "hello world");

        // Registry with existing profile
        let tr = TargetRegistry {
            name: "tmux".to_string(),
            real_path: source_file.clone(),
            profiles: vec!["work".to_string()],
        };
        let mut targets = std::collections::HashMap::new();
        targets.insert("tmux".to_string(), tr.clone());

        let registry = Registry {
            base_path: wardrobe.to_path_buf(),
            targets,
        };

        let cmd = Add {
            target: "tmux".to_string(),
            profile: "work".to_string(),
            to_register: None,
            force: true,
        };

        let target_dir = wardrobe.join("tmux");
        fs::create_dir_all(&target_dir).unwrap();

        // Should succeed because force = true
        add(&cmd, &registry).unwrap();

        let profile_path = wardrobe.join("tmux").join("work");
        assert!(profile_path.exists());
    }

    #[test]
    fn test_add_fail_without_register() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path();

        // Empty registry
        let registry = Registry {
            base_path: wardrobe.to_path_buf(),
            targets: std::collections::HashMap::new(),
        };

        let cmd = Add {
            target: "tmux".to_string(),
            profile: "work".to_string(),
            to_register: None,
            force: false,
        };

        // Should fail because target not registered
        let res = add(&cmd, &registry);
        assert!(res.is_err());
        assert!(format!("{}", res.unwrap_err()).contains("Target tmux not registered"));
    }
}

