use crate::dispatch::io;
use crate::domain::Add;
use crate::errors::RobeError;
use crate::registry::{Registry, TargetMetadata};
use std::fs;
use std::path::Path;

pub fn add(cmd: &Add, registry: &Registry) -> Result<(), RobeError> {
    if let Some(target_registry) = registry.get_target_registry(&cmd.target) {
        if target_registry.profiles.contains(&cmd.profile.to_string()) && !cmd.force {
            return Err(RobeError::message(format!(
                "Profile {}/{} already exists. Use `-f` to update.",
                &cmd.target, &cmd.profile
            )));
        }

        let robe_target_path = Path::join(&registry.base_path, &cmd.target);

        let mut real_path = target_registry.real_path.clone();

        if let Some(register_path) = cmd.to_register.clone() {
            if !cmd.force {
                return Err(RobeError::message(format!(
                    "Target {} is already registered. Use `-f` to update registered target path.",
                    &cmd.target
                )));
            }
            real_path = register_path.clone();
            io::store_metadata(&robe_target_path, &TargetMetadata::create(&register_path)?)?;
        }

        let mut target_path = registry.base_path.clone();
        target_path.push(cmd.target.clone());
        target_path.push(cmd.profile.clone());

        io::replace_file_or_dir(&real_path, &target_path)?;
    } else if let Some(register_path) = cmd.to_register.clone() {
        let target_path = Path::join(&registry.base_path, &cmd.target);
        let mut target_path = target_path.clone();
        fs::create_dir_all(&target_path)?;

        let meta = TargetMetadata::create(&register_path)?;

        io::store_metadata(&target_path, &meta)?;

        target_path.push(cmd.profile.clone());

        io::replace_file_or_dir(&register_path, &target_path)?;
    } else {
        return Err(RobeError::message(format!(
            "Target {} not registered. Use -r <file> to register.",
            &cmd.target
        )));
    }

    Ok(())
}
