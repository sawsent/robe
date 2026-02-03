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
    pub target_path: PathBuf,
    pub profiles: Vec<String>,
}

impl TargetRegistry {
    pub fn new(name: &str, meta: &TargetMetadata, profiles: &Vec<PathBuf>) -> Self {
        let mut prof: Vec<String> = Vec::new();
        for path in profiles {
            if let Some(name) = path.file_name().map(|f| f.to_string_lossy().to_string()) {
                prof.push(name);
            }
        }
        Self {
            name: name.to_string(),
            target_path: PathBuf::from(meta.target_path.clone()),
            profiles: prof,
        }
    }

    pub fn assert_profile_exists(&self, profile: &str) -> Result<(), RobeError> {
        if self.profiles.contains(&profile.to_string()) {
            Ok(())
        } else {
            Err(RobeError::message(format!(
                "Profile {}/{} not found.",
                &self.name, &profile
            )))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetMetadata {
    target_path: String,
}

impl TargetMetadata {
    pub fn create(path: &Path) -> Self {
        Self {
            target_path: path.to_string_lossy().to_string(),
        }
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
