use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::errors::RobeError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registry {
    pub base_path: PathBuf,
    pub targets: HashMap<String, TargetRegistry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetRegistry {
    pub name: String,
    pub real_path: PathBuf,
    pub profiles: Vec<String>,
}

impl TargetRegistry {
    pub fn new(name: &str, meta: &TargetMetadata, profiles: &[PathBuf]) -> Self {
        let mut prof: Vec<String> = Vec::new();
        for path in profiles {
            if let Some(name) = path.file_name().map(|f| f.to_string_lossy().to_string()) {
                prof.push(name);
            }
        }
        Self {
            name: name.to_string(),
            real_path: PathBuf::from(meta.real_path.clone()),
            profiles: prof,
        }
    }

    pub fn assert_profile_exists(&self, profile: &str) -> Result<(), RobeError> {
        if self.profiles.iter().any(|p| p == profile) {
            Ok(())
        } else {
            Err(RobeError::message(format!(
                "Profile {}/{} not found.",
                &self.name, &profile
            )))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TargetMetadata {
    real_path: String,
}

impl TargetMetadata {
    pub fn create(path: &Path) -> Result<Self, RobeError> {
        let tp = path.canonicalize()?;
        Ok(Self {
            real_path: tp.to_string_lossy().to_string(),
        })
    }
}

impl Registry {
    pub fn get_target_registry(&self, target: &str) -> Option<TargetRegistry> {
        self.targets.get(target).cloned()
    }

    pub fn target_registry(&self, target: &str) -> Result<TargetRegistry, RobeError> {
        match self.get_target_registry(target) {
            None => Err(RobeError::message(format!("Target {} not found.", target))),
            Some(tr) => Ok(tr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_target_registry_new() {
        let dir = tempdir().unwrap();

        let p1 = dir.path().join("work");
        let p2 = dir.path().join("clean");

        fs::create_dir(&p1).unwrap();
        fs::create_dir(&p2).unwrap();

        let meta = TargetMetadata {
            real_path: "/real/path".to_string(),
        };

        let tr = TargetRegistry::new("tmux", &meta, &vec![p1.clone(), p2.clone()]);

        assert_eq!(tr.name, "tmux");
        assert_eq!(tr.real_path, PathBuf::from("/real/path"));
        assert_eq!(tr.profiles.len(), 2);
        assert!(tr.profiles.contains(&"work".to_string()));
        assert!(tr.profiles.contains(&"clean".to_string()));
    }

    #[test]
    fn test_assert_profile_exists_ok() {
        let tr = TargetRegistry {
            name: "tmux".to_string(),
            real_path: PathBuf::from("/tmp"),
            profiles: vec!["work".to_string(), "clean".to_string()],
        };

        assert!(tr.assert_profile_exists("work").is_ok());
    }

    #[test]
    fn test_assert_profile_exists_error() {
        let tr = TargetRegistry {
            name: "tmux".to_string(),
            real_path: PathBuf::from("/tmp"),
            profiles: vec!["work".to_string()],
        };

        let err = tr.assert_profile_exists("missing").unwrap_err();
        let msg = format!("{}", err);

        assert!(msg.contains("Profile tmux/missing not found"));
    }

    #[test]
    fn test_target_metadata_create() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        let meta = TargetMetadata::create(path).unwrap();

        assert!(meta.real_path.contains(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn test_registry_get_target() {
        let mut reg = Registry::default();

        let tr = TargetRegistry {
            name: "tmux".to_string(),
            real_path: PathBuf::from("/tmp"),
            profiles: vec!["work".to_string()],
        };

        reg.targets.insert("tmux".to_string(), tr.clone());

        let result = reg.get_target_registry("tmux");
        assert!(result.is_some());
    }

    #[test]
    fn test_registry_target_registry_error() {
        let reg = Registry::default();

        let err = reg.target_registry("missing").unwrap_err();
        let msg = format!("{}", err);

        assert!(msg.contains("Target missing not found"));
    }
}
