use std::fs;
use std::path::{PathBuf};
use crate::errors::SuitError;
use crate::settings::Settings;


pub fn settings_file_path() -> String {
    let maybe_fp = dirs::config_local_dir()
        .map(|mut p| {
            p.push("suit");
            p.push("config.toml");
            p
        });

    maybe_fp.map(|fp| fp.to_string_lossy().to_string()).unwrap_or_else(|| "$HOME/.config/suit/config.toml".to_string())
}

pub fn get_settings(fp: &String) -> Settings {
    if let Ok(string) = fs::read_to_string(&PathBuf::from(fp)) {
        if let Ok(settings) = toml::from_str(&string) {
            return settings;
        }
    }
    Settings::default()
}

pub fn get_subdirs(dir: &PathBuf) -> Result<Vec<PathBuf>, SuitError> {
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

pub fn _get_files_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, SuitError> {
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

pub fn get_files_in_dir_except(dir: &PathBuf, file_name: &str) -> Result<Vec<PathBuf>, SuitError> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let f = entry.file_name().to_string_lossy().to_string();

        if path.is_file() && f != file_name {
            dirs.push(path);
        }
    }

    Ok(dirs)
}

