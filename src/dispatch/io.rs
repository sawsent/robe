use crate::errors::RobeError;
use crate::registry::{Registry, TargetMetadata, TargetRegistry};
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), RobeError> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn copy_file_or_dir(src: &PathBuf, dst: &PathBuf) -> Result<(), RobeError> {
    if src.is_file() {
        copy_file(src, dst)
    } else if src.is_dir() {
        copy_dir_all(src, dst)
    } else {
        Err(RobeError::Internal("Robe does not allow symlinks.".to_string()))
    }
}

pub fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), RobeError> {
    if src.is_file() {
        return copy_file(src, dst);
    }

    if !dst.exists() {
        fs::create_dir_all(&dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = Path::join(dst, entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}

pub fn store_metadata(target_path: &Path, meta: &TargetMetadata) -> Result<(), RobeError> {
    let p = Path::join(target_path, "meta.toml");
    fs::write(&p, toml::to_string_pretty(meta)?)?;
    Ok(())
}

pub fn delete_profile(
    registry: &Registry,
    target_registry: &TargetRegistry,
    profile: &str,
) -> Result<(), RobeError> {
    let mut profile_path = registry.base_path.clone();
    profile_path.push(target_registry.name.clone());
    profile_path.push(profile);

    fs::remove_file(profile_path)?;
    Ok(())
}

pub fn delete_target(target_name: &str, registry: &Registry) -> Result<(), RobeError> {
    let mut target_path = registry.base_path.clone();
    target_path.push(target_name);
    fs::remove_dir_all(target_path)?;
    Ok(())
}

pub fn print_file(fp: &Path) -> Result<(), RobeError> {
    let fstr = fs::read_to_string(fp)?;
    println!("{}", fstr);
    Ok(())
}
