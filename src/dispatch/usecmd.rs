use crate::dispatch::io;
use crate::domain::Use;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn usecmd(cmd: &Use, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;
    target_registry.assert_profile_exists(&cmd.profile)?;
    let target = target_registry.real_path;
    let from = registry.base_path.clone().join(&cmd.target).join(&cmd.profile);

    io::replace_file_or_dir(&from, &target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Use;
    use crate::registry::{Registry, TargetRegistry};
    use crate::errors::RobeError;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_usecmd_replaces_file() -> Result<(), RobeError> {
        let tmp_wardrobe = tempdir().unwrap();
        let base_path = tmp_wardrobe.path().to_path_buf();

        let tmp_target = tempdir().unwrap();
        let target_real_path = tmp_target.path().join("config.txt");
        fs::write(&target_real_path, "original content")?;

        let target_name = "myconfig";
        let profile_name = "work";
        let profile_dir = base_path.join(target_name);
        fs::create_dir_all(&profile_dir)?;
        let profile_path = profile_dir.join(profile_name);
        fs::write(&profile_path, "profile content")?;

        let mut targets = HashMap::new();
        targets.insert(
            target_name.to_string(),
            TargetRegistry {
                name: target_name.to_string(),
                real_path: target_real_path.clone(),
                profiles: vec![profile_name.to_string()],
                last_activated_profile: None,
            },
        );

        let registry = Registry {
            base_path: base_path.clone(),
            targets,
        };

        let cmd = Use {
            target: target_name.to_string(),
            profile: profile_name.to_string(),
        };

        usecmd(&cmd, &registry)?;

        let content = fs::read_to_string(&target_real_path)?;
        assert_eq!(content, "profile content");

        Ok(())
    }
}
