use crate::dispatch::io;
use crate::domain::Add;
use crate::errors::RobeError;
use crate::registry::Registry;

/// Add a profile to a target.
/// Requires force if profile already exists.
/// Fails if target does not exist.
///
/// Behaviour:
/// copy the current file/dir at `real_path` to `path/to/wardrobe/<target>/<profile>`
pub fn add(cmd: &Add, registry: &Registry) -> Result<(), RobeError> {
    if let Some(target_registry) = registry.get_target_registry(&cmd.target) {
        if target_registry.profiles.contains(&cmd.profile.to_string()) && !cmd.force {
            return Err(RobeError::message(format!(
                "Profile {}/{} already exists. Use `-f` to update.",
                &cmd.target, &cmd.profile
            )));
        }

        let mut target_path = registry.base_path.clone();
        target_path.push(cmd.target.clone());
        target_path.push(cmd.profile.clone());

        io::replace_file_or_dir(&target_registry.real_path, &target_path)?;
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
    use crate::registry::{Registry, TargetRegistry};
    use std::fs;
    use tempfile::tempdir;

    fn create_file(path: &std::path::Path, content: &str) {
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_add_existing_profile_requires_force() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path();

        // Registry with one existing profile
        let tr = TargetRegistry {
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
            force: false,
        };

        // Should fail because profile exists and no -f
        let res = add(&cmd, &registry);
        assert!(res.is_err());
        assert!(format!("{}", res.unwrap_err()).contains("Profile tmux/work already exists"));
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
            force: false,
        };

        // Should fail because target not registered
        let res = add(&cmd, &registry);
        assert!(res.is_err());
        assert!(format!("{}", res.unwrap_err()).contains("Target tmux not registered"));
    }
}
