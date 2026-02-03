use crate::dispatch::io;
use crate::domain::Use;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn usecmd(cmd: &Use, registry: &Registry) -> Result<(), RobeError> {
    let target_registry = registry.target_registry(&cmd.target)?;
    target_registry.assert_profile_exists(&cmd.profile)?;
    let target = target_registry.target_path;
    let mut from = registry.base_path.clone();
    from.push(&cmd.target);
    from.push(&cmd.profile);

    io::copy_file_or_dir(&from, &target)?;
    Ok(())
}
