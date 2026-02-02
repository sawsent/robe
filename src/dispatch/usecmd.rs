use crate::dispatch::io;
use crate::domain::Use;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn usecmd(cmd: &Use, registry: &Registry) -> Result<(), RobeError> {
    let tool_registry = registry.tool_registry(&cmd.tool)?;
    tool_registry.assert_profile_exists(&cmd.profile)?;
    let target = tool_registry.real_path;
    let mut from = registry.base_path.clone();
    from.push(&cmd.tool);
    from.push(&cmd.profile);

    io::copy_file(&from, &target)?;
    Ok(())
}
