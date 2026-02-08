use crate::domain::Edit;
use crate::errors::RobeError;
use crate::registry::Registry;
use std::env;
use std::process::Command;

pub fn edit(cmd: &Edit, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;

    let fp = match &cmd.profile {
        Some(profile) => {
            target_registry.assert_profile_exists(profile)?;
            let mut from = registry.base_path.clone();
            from.push(&cmd.target);
            from.push(profile);
            from
        }
        None => target_registry.real_path,
    };

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".into());

    Command::new(editor).arg(&fp).status()?;

    Ok(())
}
