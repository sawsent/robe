use crate::errors::RobeError;
use crate::settings::Settings;
use std::fs;
use std::path::PathBuf;

pub fn settings_file_path() -> String {
    let maybe_fp = dirs::config_local_dir().map(|mut p| {
        p.push("robe");
        p.push("config.toml");
        p
    });

    maybe_fp
        .map(|fp| fp.to_string_lossy().to_string())
        .unwrap_or_else(|| "$HOME/.config/robe/config.toml".to_string())
}

pub fn get_settings(fp: &String) -> Settings {
    if let Ok(string) = fs::read_to_string(PathBuf::from(fp))
        && let Ok(settings) = toml::from_str(&string)
    {
        return settings;
    }
    Settings::default()
}

pub fn get_subdirs(dir: &PathBuf) -> Result<Vec<PathBuf>, RobeError> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

pub fn _get_files_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, RobeError> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

pub fn get_profiles_from_dir(dir: &PathBuf, file_name: &str) -> Result<Vec<PathBuf>, RobeError> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let f = entry.file_name().to_string_lossy().to_string();

        if f != file_name {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_settings_file_path_contains_expected_suffix() {
        let path = settings_file_path();
        assert!(path.ends_with("robe/config.toml"));
    }

    #[test]
    fn test_get_settings_valid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "wardrobe = \"/tmp/robe\"").unwrap();

        let settings = get_settings(&file_path.to_string_lossy().to_string());

        assert_eq!(settings.wardrobe, "/tmp/robe");
    }

    #[test]
    fn test_get_settings_invalid_file_returns_default() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        fs::write(&file_path, "invalid toml :::").unwrap();

        let settings = get_settings(&file_path.to_string_lossy().to_string());

        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn test_get_settings_missing_file_returns_default() {
        let settings = get_settings(&"nonexistent.toml".to_string());
        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn test_get_subdirs() {
        let dir = tempdir().unwrap();

        let sub1 = dir.path().join("a");
        let sub2 = dir.path().join("b");
        let file = dir.path().join("file.txt");

        fs::create_dir(&sub1).unwrap();
        fs::create_dir(&sub2).unwrap();
        File::create(&file).unwrap();

        let mut result = get_subdirs(&dir.path().to_path_buf()).unwrap();
        result.sort();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&sub1));
        assert!(result.contains(&sub2));
    }

    #[test]
    fn test_get_files_in_dir() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        let sub = dir.path().join("sub");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        fs::create_dir(&sub).unwrap();

        let mut result = _get_files_in_dir(&dir.path().to_path_buf()).unwrap();
        result.sort();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&file1));
        assert!(result.contains(&file2));
    }

    #[test]
    fn test_get_profiles_from_dir_excludes_filename() {
        let dir = tempdir().unwrap();

        let keep = dir.path().join("profile1");
        let exclude = dir.path().join("meta.toml");

        fs::create_dir(&keep).unwrap();
        File::create(&exclude).unwrap();

        let result = get_profiles_from_dir(&dir.path().to_path_buf(), "meta.toml").unwrap();

        assert_eq!(result.len(), 1);
        assert!(result.contains(&keep));
        assert!(!result.contains(&exclude));
    }
}
