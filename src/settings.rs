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
        .unwrap_or_else(|| PathBuf::from("$HOME/.local/share/robe/wardrobe"))
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let s = Settings::default();

        assert!(!s.wardrobe.is_empty());
        assert!(s.wardrobe.contains("robe"));
    }

    #[test]
    fn test_deserialize_settings() {
        let toml = r#"
wardrobe = "/tmp/my-robe"
"#;

        let settings: Settings = toml::from_str(toml).unwrap();

        assert_eq!(settings.wardrobe, "/tmp/my-robe");
    }

    #[test]
    fn test_deserialize_equals_struct() {
        let toml = r#"
wardrobe = "/tmp/test"
"#;

        let parsed: Settings = toml::from_str(toml).unwrap();

        let expected = Settings {
            wardrobe: "/tmp/test".to_string(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_default_data_location_contains_robe() {
        let path = default_data_location();
        assert!(path.contains("robe"));
    }
}
