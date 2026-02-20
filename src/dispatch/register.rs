use crate::dispatch::io;
use crate::domain::Register;
use crate::errors::RobeError;
use crate::registry::{Registry, TargetMetadata};

/// Register a target with a profile.
/// Fails if target already exists.
///
/// Behaviour:
/// store target metadata
/// copy the current file/dir at `real_path` to `path/to/wardrobe/<target>/<profile>`
pub fn register(cmd: &Register, registry: &Registry) -> Result<(), RobeError> {
    if registry.get_target_registry(&cmd.target).is_some() {
        return Err(RobeError::message(format!(
            "target {} already exists.",
            &cmd.target
        )));
    }

    let new_meta = TargetMetadata::create(&cmd.register_file_path)?;

    io::store_metadata(registry, &new_meta, &cmd.target)?;
    io::replace_file_or_dir(
        &cmd.register_file_path,
        &registry.base_path.join(&cmd.target).join(&cmd.profile),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Register;
    use crate::registry::{Registry, TargetRegistry};
    use std::collections::HashMap;
    use std::fs;
    use tempfile::tempdir;

    fn create_file(path: &std::path::Path, content: &str) {
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_register_new_target() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path().join("wardrobe");
        fs::create_dir_all(&wardrobe).unwrap();

        // file to register
        let source = temp.path().join("config.txt");
        create_file(&source, "hello");

        let registry = Registry {
            base_path: wardrobe.clone(),
            targets: HashMap::new(),
        };

        let cmd = Register {
            target: "target".to_string(),
            profile: "profile".to_string(),
            register_file_path: source.clone(),
        };

        register(&cmd, &registry).unwrap();

        // profile should exist
        let profile = wardrobe.join("target").join("profile");
        assert!(profile.exists());

        let content = fs::read_to_string(profile).unwrap();
        assert_eq!(content, "hello");

        // metadata should exist
        let meta = wardrobe.join("target").join("meta.toml");
        assert!(meta.exists());
    }

    #[test]
    fn test_register_existing_target_fails() {
        let temp = tempdir().unwrap();
        let wardrobe = temp.path().join("wardrobe");
        fs::create_dir_all(&wardrobe).unwrap();

        let mut targets = HashMap::new();
        targets.insert(
            "target".to_string(),
            TargetRegistry {
                name: "target".to_string(),
                real_path: temp.path().to_path_buf(),
                profiles: vec![],
            },
        );

        let registry = Registry {
            base_path: wardrobe,
            targets,
        };

        let cmd = Register {
            target: "target".to_string(),
            profile: "profile".to_string(),
            register_file_path: temp.path().join("file"),
        };

        let result = register(&cmd, &registry);

        assert!(result.is_err());
    }
}
