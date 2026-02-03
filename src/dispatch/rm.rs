use crate::dispatch::io;
use crate::domain::Rm;
use crate::errors::RobeError;
use crate::registry::{Registry, TargetRegistry};

pub fn rm(cmd: &Rm, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;

    match &cmd.profile {
        Some(profile) => rm_profile(&target_registry, profile, registry)?,
        None => rm_target(&target_registry.name, registry)?,
    }

    Ok(())
}

fn rm_profile(
    target_registry: &TargetRegistry,
    profile: &str,
    registry: &Registry,
) -> Result<(), RobeError> {
    target_registry.assert_profile_exists(profile)?;

    io::delete_profile(registry, target_registry, profile)?;
    Ok(())
}

fn rm_target(target: &str, registry: &Registry) -> Result<(), RobeError> {
    io::delete_target(target, registry)?;
    Ok(())
}
