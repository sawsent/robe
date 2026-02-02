use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub data_location: String
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            data_location: default_data_location()
        }
    }
}

fn default_data_location() -> String {
    dirs::data_local_dir()
        .map(|mut p| {
            p.push("suit");
            p
        })
        .unwrap_or_else(|| PathBuf::from("$HOME/.config/suit/config.toml")).to_string_lossy().to_string()
}
