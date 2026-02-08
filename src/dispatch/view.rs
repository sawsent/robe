use crate::domain::View;
use crate::errors::RobeError;
use crate::registry::Registry;
use std::fs;
use std::path::Path;

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

    if cmd.raw {
        view_file_or_dir_raw(&fp)?;
    } else {
        view_file_or_dir(&fp)?;
    }

    Ok(())
}

fn view_file_or_dir(fp: &Path) -> Result<(), RobeError> {
    if fp.is_dir() {
        println!("Directory: {}\n", fp.display());
        view_dir_raw(fp)?;
        println!("\nPath: {}", fp.display());
    } else {
        println!("File: {}\n", fp.display());
        println!("------------------------------\n");
        view_file_raw(fp)?;
        println!("------------------------------");
        println!("Path: {}", fp.display());
    }

    Ok(())
}

fn view_file_or_dir_raw(fp: &Path) -> Result<(), RobeError> {
    if fp.is_dir() {
        view_dir_raw(fp)
    } else {
        view_file_raw(fp)
    }
}

fn view_file_raw(fp: &Path) -> Result<(), RobeError> {
    println!("{}", fs::read_to_string(fp)?);
    Ok(())
}

fn view_dir_raw(fp: &Path) -> Result<(), RobeError> {
    let mut entries: Vec<_> = fs::read_dir(fp)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let f = entry.file_name();
        let fname = f.to_string_lossy();
        let marker = if entry.path().is_dir() { "/" } else { "" };
        println!("{}{}", fname, marker);
    }

    Ok(())
}
