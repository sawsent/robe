use crate::dispatch::io;
use crate::domain::Rm;
use crate::errors::RobeError;
use crate::registry::{Registry, ToolRegistry};

pub fn rm(cmd: &Rm, registry: &Registry) -> Result<(), RobeError> {
    let tool_registry = registry.tool_registry(&cmd.tool)?;

    match &cmd.profile {
        Some(profile) => rm_profile(&tool_registry, profile, registry)?,
        None => rm_tool(&tool_registry.name, registry)?,
    }

    Ok(())
}

fn rm_profile(
    tool_registry: &ToolRegistry,
    profile: &str,
    registry: &Registry,
) -> Result<(), RobeError> {
    tool_registry.assert_profile_exists(profile)?;

    io::delete_profile(registry, tool_registry, profile)?;
    Ok(())
}

fn rm_tool(tool: &str, registry: &Registry) -> Result<(), RobeError> {
    io::delete_tool(tool, registry)?;
    Ok(())
}
