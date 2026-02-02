use crate::errors::SuitError;
use crate::registry::{ToolMetadata, Registry, ToolRegistry};
use std::path::{PathBuf, Path};
use std::fs;

pub fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), SuitError> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn store_metadata(tool_path: &PathBuf, meta: &ToolMetadata) -> Result<(), SuitError> {
    let p = Path::join(tool_path, "meta.toml");
    fs::write(&p, toml::to_string_pretty(meta)?)?;
    Ok(())
}

pub fn delete_profile(registry: &Registry, tool_registry: &ToolRegistry, profile: &str) -> Result<(), SuitError> {
    let mut profile_path = registry.base_path.clone();
    profile_path.push(tool_registry.name.clone());
    profile_path.push(profile);

    fs::remove_file(profile_path)?;
    Ok(())
}
     
pub fn delete_tool(tool_name: &str, registry: &Registry) -> Result<(), SuitError> {
    let mut tool_path = registry.base_path.clone();
    tool_path.push(tool_name);
    fs::remove_dir_all(tool_path)?;
    Ok(())
}
