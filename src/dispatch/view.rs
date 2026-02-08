use crate::dispatch::io;
use crate::domain::View;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn view(cmd: &View, registry: &Registry) -> Result<(), RobeError> {
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

    if fp.is_dir() {
        println!(
            "Robe does not support viewing directory style targets natively. View with your own editor:"
        );
        println!("{:?}", fp);
    } else {
        println!("------------------------------\n");
        io::print_file(&fp)?;
        println!("------------------------------");
        println!("{:?}", fp);
    }

    Ok(())
}
