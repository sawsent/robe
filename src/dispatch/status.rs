use crate::{dispatch::hashing, domain::Status, errors::RobeError, registry::Registry};

struct TargetStatus {
    name: String,
    last_activated_profile: Option<String>,
    modified: bool,
}

impl TargetStatus {
    pub fn format(&self, max_len: usize) -> String {
        let mut out = "".to_string();
        out.push_str(&self.name);

        if max_len > self.name.len() {
            for _ in 0..(max_len - self.name.len()) {
                out.push(' ');
            }
        }

        out.push_str(" → ");

        match &self.last_activated_profile {
            Some(lap) => out.push_str(lap),
            None => out.push_str("(none)"),
        }

        if self.modified {
            out.push_str(" * modified");
        }

        out
    }
}


pub fn status(cmd: &Status, registry: &Registry) -> Result<(), RobeError> {

    let formatted = match &cmd.target {
        Some(target) => target_status(&target, registry, 0)?,
        None => full_status(registry)?,
    };

    println!("{}", formatted);
    Ok(())
}

fn target_status(target: &str, registry: &Registry, max_size: usize) -> Result<String, RobeError> {
    let t_reg: crate::registry::TargetRegistry = registry.target_registry(target)?;
    let equal = if let Some(wardrobe_target_profile) = t_reg.last_activated_profile.clone().map(|p| registry.get_profile_path(target, &p)) {
        let wtp = wardrobe_target_profile?;
        hashing::are_paths_equal(&t_reg.real_path, &wtp)?
    } else {
        true
    };

    let t_status = TargetStatus {
        name: target.into(),
        last_activated_profile: t_reg.last_activated_profile.clone(),
        modified: !equal
    };

    Ok(t_status.format(max_size))
}

fn full_status(registry: &Registry) -> Result<String, RobeError> {
    let max_size = registry.targets.keys().max_by_key(|k| k.len()).map(|p| p.len()).unwrap_or_else(|| 0);
    let mut formatted: Vec<String> = Vec::new();

    for t in registry.targets.keys() {
        formatted.push(target_status(t, registry, max_size)?);
    }

    formatted.sort();

    Ok(formatted.join("\n"))
}
