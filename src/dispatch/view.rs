use crate::dispatch::io;
use crate::domain::View;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn view(cmd: &View, registry: &Registry) -> Result<(), RobeError> {
    let tool_registry = registry.tool_registry(&cmd.tool)?;

    let fp = match &cmd.profile {
        Some(profile) => {
            tool_registry.assert_profile_exists(profile)?;
            let mut from = registry.base_path.clone();
            from.push(&cmd.tool);
            from.push(profile);
            from
        }
        None => tool_registry.real_path,
    };

    io::print_file(&fp)?;

    Ok(())
}
