use crate::errors::RobeError;
use crate::registry::{Registry, ToolMetadata, ToolRegistry};
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), RobeError> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn store_metadata(tool_path: &PathBuf, meta: &ToolMetadata) -> Result<(), RobeError> {
    let p = Path::join(tool_path, "meta.toml");
    fs::write(&p, toml::to_string_pretty(meta)?)?;
    Ok(())
}

pub fn delete_profile(
    registry: &Registry,
    tool_registry: &ToolRegistry,
    profile: &str,
) -> Result<(), RobeError> {
    let mut profile_path = registry.base_path.clone();
    profile_path.push(tool_registry.name.clone());
    profile_path.push(profile);

    fs::remove_file(profile_path)?;
    Ok(())
}

pub fn delete_tool(tool_name: &str, registry: &Registry) -> Result<(), RobeError> {
    let mut tool_path = registry.base_path.clone();
    tool_path.push(tool_name);
    fs::remove_dir_all(tool_path)?;
    Ok(())
}
