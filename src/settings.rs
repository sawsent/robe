use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Settings {
    pub wardrobe: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            wardrobe: default_data_location(),
        }
    }
}

fn default_data_location() -> String {
    dirs::data_local_dir()
        .map(|mut p| {
            p.push("robe");
            p
        })
        .unwrap_or_else(|| PathBuf::from("$HOME/.config/robe/config.toml"))
        .to_string_lossy()
        .to_string()
}
