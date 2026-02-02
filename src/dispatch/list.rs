use crate::domain::List;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn list(cmd: &List, registry: &Registry) -> Result<(), RobeError> {
    match &cmd.tool {
        Some(t) => {
            let tr = registry.tool_registry(t)?;
            println!("robes for {}:", t);
            for p in tr.profiles {
                println!("  - {}", p)
            }
        }
        None => {
            println!("registered tools:");
            for t in registry.tools.keys() {
                println!("  - {}", t);
            }
        }
    }

    Ok(())
}
