use crate::domain::List;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn list(cmd: &List, registry: &Registry) -> Result<(), RobeError> {
    match &cmd.target {
        Some(t) => {
            let tr = registry.target_registry(t)?;
            println!("robes for {}:", t);
            for p in tr.profiles {
                println!("  - {}", p)
            }
        }
        None => {
            println!("registered targets:");
            for t in registry.targets.keys() {
                println!("  - {}", t);
            }
        }
    }

    Ok(())
}
