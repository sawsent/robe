use crate::domain::List;
use crate::errors::RobeError;
use crate::registry::Registry;

pub fn list(cmd: &List, registry: &Registry) -> Result<(), RobeError> {
    let formatted = match &cmd.target {
        Some(t) => {
            let tr = registry.target_registry(t)?;
            format_profiles(t, &tr.profiles)
        }
        None => {
            let targets: Vec<_> = registry.targets.keys().collect();
            format_targets(&targets)
        }
    };

    println!("{}", formatted);

    Ok(())
}

fn format_targets(targets: &[&String]) -> String {
    let mut out = "".to_string();
    out.push_str("Registered targets:\n");
    targets.iter().for_each(|t| out.push_str(&format!("  - {}", t)));
    out
}

fn format_profiles(target: &str, profiles: &[String]) -> String {
    let mut out = "".to_string();
    out.push_str(&format!("Robes for {}:\n", target));
    profiles.iter().for_each(|p| out.push_str(&format!("  - {}", p)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::collections::HashMap;

    #[test]
    fn test_list_all_targets() {
        let mut registry = Registry {
            base_path: tempdir().unwrap().path().to_path_buf(),
            targets: HashMap::new(),
        };
        registry.targets.insert(
            "tmux".to_string(),
            crate::registry::TargetRegistry {
                name: "tmux".to_string(),
                real_path: std::path::PathBuf::from("/fake/tmux"),
                profiles: vec!["work".to_string(), "clean".to_string()],
                last_activated_profile: None,
            },
        );
        registry.targets.insert(
            "nvim".to_string(),
            crate::registry::TargetRegistry {
                name: "nvim".to_string(),
                real_path: std::path::PathBuf::from("/fake/nvim"),
                profiles: vec!["minimal".to_string()],
                last_activated_profile: None,
            },
        );

        let cmd = List { target: None };

        let result = list(&cmd, &registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_profiles_for_target() {
        let mut registry = Registry {
            base_path: tempdir().unwrap().path().to_path_buf(),
            targets: HashMap::new(),
        };
        registry.targets.insert(
            "tmux".to_string(),
            crate::registry::TargetRegistry {
                name: "tmux".to_string(),
                real_path: std::path::PathBuf::from("/fake/tmux"),
                profiles: vec!["work".to_string(), "clean".to_string()],
                last_activated_profile: None,
            },
        );

        let cmd = List { target: Some("tmux".to_string()) };

        let result = list(&cmd, &registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_targets() {
        let mut targets_map: HashMap<String, ()> = std::collections::HashMap::new();
        targets_map.insert("t1".to_string(), ());
        targets_map.insert("t2".to_string(), ());
        targets_map.insert("t3".to_string(), ());

        let targets: Vec<_> = targets_map.keys().collect();

        let result = format_targets(&targets);

        assert!(result.contains("t1"));
        assert!(result.contains("t2"));
        assert!(result.contains("t3"));
        assert!(result.contains("Registered targets"));
    }

    #[test]
    fn test_format_profiles() {
        let targets: Vec<String> = vec!["work".to_string(), "minimal".to_string()];

        let result = format_profiles("tmux", &targets);

        assert!(result.contains("work"));
        assert!(result.contains("minimal"));
        assert!(result.contains("Robes for tmux"));
    }
}
