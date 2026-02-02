use crate::errors::SuitError;
use crate::domain::List;
use crate::registry::Registry;

pub fn list(cmd: &List, registry: &Registry) -> Result<(), SuitError> {

    match &cmd.tool {
        Some(t) => {
            let tr = registry.tool_registry(t)?;
            println!("suits for {}:", t);
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

