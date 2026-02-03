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
