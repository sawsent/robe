use crate::dispatch::io;
use crate::domain::Rm;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn rm(cmd: &Rm, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;

    match &cmd.profile {
        Some(profile) => {
            target_registry.assert_profile_exists(profile)?;
            io::delete_profile(registry, &target_registry, profile)
        }
        None => io::delete_target(&cmd.target, registry),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Rm;
    use crate::registry::{Registry, TargetRegistry};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn setup_registry(real_file_contents: &String) -> (tempfile::TempDir, Registry, PathBuf) {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path().join("wardrobe");
        let real = temp.path().join("real");
        fs::create_dir_all(&wardrobe).unwrap();
        fs::create_dir_all(&real).unwrap();

        let real_target_path = real.join("file").to_path_buf();
        fs::write(&real_target_path, &real_file_contents).unwrap();

        let wardrobe_target_dir = wardrobe.join("tmux");
        fs::create_dir_all(&wardrobe_target_dir).unwrap();

        // profiles
        fs::write(wardrobe_target_dir.join("work"), "a").unwrap();
        fs::write(wardrobe_target_dir.join("home"), "b").unwrap();

        let target_registry = TargetRegistry {
            name: "tmux".to_string(),
            real_path: PathBuf::from("/fake/path"),
            profiles: vec!["work".to_string(), "home".to_string()],
        };

        let mut targets = HashMap::new();
        targets.insert("tmux".to_string(), target_registry);

        let registry = Registry {
            base_path: wardrobe,
            targets,
        };

        (temp, registry, real_target_path)
    }

    #[test]
    fn test_rm_profile() {
        let real_file_contents: String = "content".into();
        let (_temp, registry, real_target_path) = setup_registry(&real_file_contents);

        let cmd = Rm {
            target: "tmux".to_string(),
            profile: Some("work".to_string()),
        };

        rm(&cmd, &registry).unwrap();

        let profile_path = registry.base_path.join("tmux").join("work");
        assert!(!profile_path.exists());

        // other profile should still exist
        let other = registry.base_path.join("tmux").join("home");
        assert!(other.exists());
        // Make sure the real file is unchanged
        assert_eq!(
            fs::read_to_string(&real_target_path).unwrap(),
            real_file_contents
        );
    }

    #[test]
    fn test_rm_target() {
        let real_file_contents: String = "content".into();
        let (_temp, registry, real_target_path) = setup_registry(&real_file_contents);

        let cmd = Rm {
            target: "tmux".to_string(),
            profile: None,
        };

        rm(&cmd, &registry).unwrap();

        let target_path = registry.base_path.join("tmux");
        assert!(!target_path.exists());
        assert!(real_target_path.exists());
        // Make sure the real file is unchanged
        assert_eq!(
            fs::read_to_string(&real_target_path).unwrap(),
            real_file_contents
        );
    }

    #[test]
    fn test_rm_missing_profile_fails() {
        let real_file_contents: String = "content".into();
        let (_temp, registry, real_target_path) = setup_registry(&real_file_contents);

        let cmd = Rm {
            target: "tmux".to_string(),
            profile: Some("does_not_exist".to_string()),
        };

        let result = rm(&cmd, &registry);
        assert!(result.is_err());
        // Make sure the real file is unchanged
        assert_eq!(
            fs::read_to_string(&real_target_path).unwrap(),
            real_file_contents
        );
    }

    #[test]
    fn test_rm_missing_target_fails() {
        let real_file_contents: String = "content".into();
        let (_temp, registry, real_target_path) = setup_registry(&real_file_contents);

        let cmd = Rm {
            target: "ghost".to_string(),
            profile: None,
        };

        let result = rm(&cmd, &registry);
        assert!(result.is_err());
        // Make sure the real file is unchanged
        assert_eq!(
            fs::read_to_string(&real_target_path).unwrap(),
            real_file_contents
        );
    }
}
