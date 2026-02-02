use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::errors::SuitError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registry {
    pub base_path: PathBuf,
    pub tools: HashMap<String, ToolRegistry>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolRegistry {
    pub name: String,
    pub real_path: PathBuf,
    pub profiles: Vec<String>,
}

impl ToolRegistry {
    pub fn new(name: &str, meta: &ToolMetadata, profiles: &Vec<PathBuf>) -> Self {
        let mut prof: Vec<String> = Vec::new();
        for path in profiles {
            if let Some(name) = path.file_name().map(|f| f.to_string_lossy().to_string()) {
                prof.push(name);
            }
        }
        Self {
            name: name.to_string(),
            real_path: PathBuf::from(meta.real_path.clone()),
            profiles: prof
        }
    }

    pub fn assert_profile_exists(&self, profile: &str) -> Result<(), SuitError> {
        if self.profiles.contains(&profile.to_string()) {
            Ok(())
        } else {
            Err(SuitError::message(format!("Profile {}/{} not found.", &self.name, &profile)))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolMetadata {
    real_path: String,
}

impl ToolMetadata {
    pub fn create(path: &PathBuf) -> Self {
        Self {
            real_path: path.to_string_lossy().to_string()
        }
    }
}

impl Registry {
    pub fn get_tool_registry(&self, tool: &str) -> Option<ToolRegistry> {
        self.tools.get(&tool.to_string()).cloned()
    }

    pub fn tool_registry(&self, tool: &str) -> Result<ToolRegistry, SuitError> {
        match self.get_tool_registry(tool) {
            None => Err(SuitError::message(format!("Tool {} not found.", tool))),
            Some(tr) => Ok(tr)
        }
    }
}

