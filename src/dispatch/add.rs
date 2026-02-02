use crate::errors::RobeError;
use crate::domain::Add;
use crate::registry::{Registry, ToolMetadata};
use crate::dispatch::io;
use std::path::{Path};
use std::fs;

pub fn add(cmd: &Add, registry: &Registry) -> Result<(), RobeError> {
    if let Some(tool_registry) = registry.get_tool_registry(&cmd.tool) {
        if tool_registry.profiles.contains(&cmd.profile.to_string()) && !cmd.force {
            return Err(RobeError::message(format!("Profile {}/{} already exists. Use `-f` to update.", &cmd.tool, &cmd.profile)));
        }

        let tool_path = Path::join(&registry.base_path, &cmd.tool);

        let mut real_path = tool_registry.real_path.clone();

        if let Some(register_path) = cmd.to_register.clone() {
            real_path = register_path.clone();
            io::store_metadata(&tool_path, &ToolMetadata::create(&register_path))?;
        }

        let mut target_path = registry.base_path.clone();
        target_path.push(cmd.tool.clone());
        target_path.push(cmd.profile.clone());

        io::copy_file(&real_path, &target_path)?;
    } else {

        if let Some(register_path) = cmd.to_register.clone() {
            let tool_path = Path::join(&registry.base_path, &cmd.tool);
            let mut target_path = tool_path.clone();
            fs::create_dir_all(&tool_path)?;

            let meta = ToolMetadata::create(&register_path);

            io::store_metadata(&tool_path, &meta)?;

            target_path.push(cmd.profile.clone());

            io::copy_file(&register_path, &target_path)?;
        } else {
            return Err(RobeError::message(format!("Tool {} not registered. Use -r <file> to register.", &cmd.tool)));
        }
    }

    Ok(())
}

