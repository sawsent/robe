use crate::errors::RobeError;
use crate::registry::{Registry, TargetMetadata, TargetRegistry};
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), RobeError> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn copy_file_or_dir(from: &PathBuf, to: &PathBuf) -> Result<(), RobeError> {
    if from.is_file() {
        copy_file(from, to)
    } else if from.is_dir() {
        copy_dir_all(from, to)
    } else {
        Err(RobeError::Internal(
            "Robe does not allow symlinks.".to_string(),
        ))
    }
}

pub fn copy_dir_all(from: &PathBuf, to: &PathBuf) -> Result<(), RobeError> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = Path::join(to, entry.file_name());

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

    if profile_path.is_file() {
        fs::remove_file(&profile_path)?;
    }
    if profile_path.is_dir() {
        fs::remove_dir_all(&profile_path)?;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_copy_file() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let src = dir.path().join("a.txt");
        let dst = dir.path().join("b.txt");

        fs::write(&src, "hello")?;
        copy_file(&src, &dst)?;

        assert!(dst.exists());
        assert_eq!(fs::read_to_string(dst)?, "hello");
        Ok(())
    }

    #[test]
    fn test_copy_dir_all() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let src = dir.path().join("src");
        let dst = dir.path().join("dst");

        fs::create_dir_all(&src)?;
        fs::write(src.join("f1.txt"), "file1")?;
        fs::create_dir(src.join("sub"))?;
        fs::write(src.join("sub").join("f2.txt"), "file2")?;

        copy_dir_all(&src, &dst)?;

        assert!(dst.exists());
        assert!(dst.join("f1.txt").exists());
        assert_eq!(fs::read_to_string(dst.join("f1.txt"))?, "file1");
        assert!(dst.join("sub").join("f2.txt").exists());
        assert_eq!(fs::read_to_string(dst.join("sub").join("f2.txt"))?, "file2");
        Ok(())
    }

    #[test]
    fn test_copy_file_or_dir_file() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let src = dir.path().join("f.txt");
        let dst = dir.path().join("g.txt");

        fs::write(&src, "hi")?;
        copy_file_or_dir(&src, &dst)?;

        assert_eq!(fs::read_to_string(dst)?, "hi");
        Ok(())
    }

    #[test]
    fn test_copy_file_or_dir_dir() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let src = dir.path().join("src");
        let dst = dir.path().join("dst");

        fs::create_dir_all(src.join("sub"))?;
        fs::write(src.join("sub").join("f.txt"), "hi")?;

        copy_file_or_dir(&src, &dst)?;

        assert!(dst.join("sub").join("f.txt").exists());
        assert_eq!(fs::read_to_string(dst.join("sub").join("f.txt"))?, "hi");
        Ok(())
    }

    #[test]
    fn test_delete_profile_file() -> Result<(), RobeError> {
        let wardrobe = tempdir()?;

        let target_name = "target";
        let profile_name = "profile";

        let mut target_dir = wardrobe.path().to_path_buf();
        target_dir.push(target_name);

        let mut profile_path = target_dir.clone();
        profile_path.push(profile_name);

        let target_registry = crate::registry::TargetRegistry {
            name: target_name.into(),
            real_path: PathBuf::from("_"),
            profiles: vec![profile_name.to_string()],
        };

        let mut targets = HashMap::new();
        targets.insert(target_name.to_string(), target_registry.clone());

        let registry = crate::registry::Registry {
            base_path: wardrobe.path().to_path_buf(),
            targets: targets,
        };

        delete_profile(&registry, &target_registry, profile_name)?;

        assert!(!profile_path.exists());
        Ok(())
    }

    #[test]
    fn test_delete_target() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let target_path = dir.path().join("target");
        fs::create_dir_all(&target_path)?;

        let registry = crate::registry::Registry {
            base_path: dir.path().to_path_buf(),
            targets: HashMap::new(),
        };

        delete_target("target", &registry)?;

        assert!(!target_path.exists());
        Ok(())
    }

    #[test]
    fn test_print_file() -> Result<(), RobeError> {
        let dir = tempdir()?;
        let f = dir.path().join("f.txt");
        fs::write(&f, "hello world")?;

        // just run to check no panic; capturing stdout is more complex
        print_file(&f)?;
        Ok(())
    }
}
