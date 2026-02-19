use crate::errors::RobeError;
use std::path::PathBuf;

pub fn parse_cmd(args: &[String]) -> Result<Command, RobeError> {
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        return Ok(Command::Help(args.join(" ")));
    }
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        return Ok(Command::Version);
    }
    if let Some(cmd) = args.first() {
        parse_internal(cmd, args.get(1..).unwrap_or(&[]))
    } else {
        Err(RobeError::BadUsage("No command provided.".to_string()))
    }
}

fn parse_internal(cmd: &str, args: &[String]) -> Result<Command, RobeError> {
    match cmd {
        "add" => Add::parse(args),
        "edit" => Edit::parse(args),
        "use" => Use::parse(args),
        "list" => List::parse(args, "list"),
        "ls" => List::parse(args, "ls"),
        "rm" => Rm::parse(args),
        "view" => View::parse(args),
        other => Err(RobeError::BadUsage(format!(
            "Command not recognized: {}",
            other
        ))),
    }
}

fn split_target_and_profile<F>(joined: &str, bad_usage: F) -> Result<(String, String), RobeError>
where
    F: Fn() -> RobeError,
{
    let split: Vec<&str> = joined.split('/').collect();
    if split.len() == 2
        && let (Some(t), Some(p)) = (split.first(), split.get(1))
    {
        return Ok((t.to_string(), p.to_string()));
    }
    Err(bad_usage())
}

#[derive(Debug, Clone)]
pub enum Command {
    Help(String),
    Version,
    Add(Add),
    Edit(Edit),
    Use(Use),
    View(View),
    List(List),
    Rm(Rm),
}

#[derive(Debug, Clone, Default)]
pub struct Add {
    pub target: String,
    pub profile: String,
    pub to_register: Option<PathBuf>,
    pub force: bool,
}

impl Add {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe add <target>/<profile> [-r file] [-f]".to_string())
    }

    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        let mut cmd = Add::default();
        let mut i = 0;

        if let Some(j) = args.get(i) {
            let (target, profile) = split_target_and_profile(j, Add::bu)?;
            cmd.target = target;
            cmd.profile = profile;
            i += 1;
        } else {
            return Err(Add::bu());
        }

        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-r" | "--register" => {
                    i += 1;
                    if let Some(f) = args.get(i) {
                        cmd.to_register = Some(PathBuf::from(&f));
                    } else {
                        return Err(Add::bu());
                    }
                }
                "-f" | "--force" => cmd.force = true,
                _ => return Err(Add::bu()),
            }
            i += 1;
        }

        Ok(Command::Add(cmd))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Edit {
    pub target: String,
    pub profile: Option<String>,
}

impl Edit {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe edit <target>[/<profile>]".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() || args.len() != 1 {
            return Err(Self::bu());
        }

        let first = args[0].clone();

        let (target, profile) = if first.contains('/') {
            let (t, p) = split_target_and_profile(&first, Self::bu)?;
            (t, Some(p))
        } else {
            (first, None)
        };

        Ok(Command::Edit(Edit { target, profile }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct View {
    pub target: String,
    pub profile: Option<String>,
    pub raw: bool,
}

impl View {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe view <target>[/<profile>] [--raw]".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() || args.len() > 2 {
            return Err(Self::bu());
        }

        let first = args[0].clone();

        let (target, profile) = if first.contains('/') {
            let (t, p) = split_target_and_profile(&first, Self::bu)?;
            (t, Some(p))
        } else {
            (first, None)
        };

        let mut raw = false;
        if let Some(r) = args.get(1) {
            match r.as_str() {
                "--raw" => raw = true,
                _ => return Err(RobeError::BadUsage(Self::bu().to_string())),
            }
        }

        Ok(Command::View(View {
            target,
            profile,
            raw,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Use {
    pub target: String,
    pub profile: String,
}

impl Use {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe use <target>/<profile>".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() || args.len() != 1 {
            Err(Self::bu())
        } else {
            let first = args[0].clone();
            let (target, profile) = split_target_and_profile(&first, Self::bu)?;
            Ok(Command::Use(Use { target, profile }))
        }
    }
}

#[derive(Debug, Clone)]
pub struct List {
    pub target: Option<String>,
}

impl List {
    fn bu(cmd: &str) -> RobeError {
        RobeError::BadUsage(format!("Usage: robe {} [<target>]", cmd).to_string())
    }
    pub fn parse(args: &[String], c: &str) -> Result<Command, RobeError> {
        if args.len() > 1 {
            Err(Self::bu(c))
        } else {
            let target = args.first().cloned();
            Ok(Command::List(List { target }))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rm {
    pub target: String,
    pub profile: Option<String>,
}

impl Rm {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe rm <target>[/<profile>]".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() || args.len() != 1 {
            return Err(Self::bu());
        }

        let first = args[0].clone();

        let (target, profile) = if first.contains('/') {
            let (t, p) = split_target_and_profile(&first, Self::bu)?;
            (t, Some(p))
        } else {
            (first, None)
        };

        Ok(Command::Rm(Rm { target, profile }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_vec(args: &[&str]) -> Result<Command, RobeError> {
        parse_cmd(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    // ---------- ADD ----------

    #[test]
    fn test_add_basic() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work"]).unwrap() {
            assert_eq!(a.target, "tmux");
            assert_eq!(a.profile, "work");
            assert!(a.to_register.is_none());
            assert!(!a.force);
        } else {
            panic!("Expected Add");
        }
    }

    #[test]
    fn test_add_force() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work", "-f"]).unwrap() {
            assert!(a.force);
        }
    }

    #[test]
    fn test_add_register() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work", "-r", "file.conf"]).unwrap() {
            assert_eq!(a.to_register.unwrap(), PathBuf::from("file.conf"));
        }
    }

    // ---------- USE ----------

    #[test]
    fn test_use() {
        if let Command::Use(u) = parse_vec(&["use", "tmux/work"]).unwrap() {
            assert_eq!(u.target, "tmux");
            assert_eq!(u.profile, "work");
        }
    }

    // ---------- EDIT ----------

    #[test]
    fn test_edit_target_only() {
        if let Command::Edit(e) = parse_vec(&["edit", "tmux"]).unwrap() {
            assert_eq!(e.target, "tmux");
            assert!(e.profile.is_none());
        }
    }

    #[test]
    fn test_edit_profile() {
        if let Command::Edit(e) = parse_vec(&["edit", "tmux/work"]).unwrap() {
            assert_eq!(e.target, "tmux");
            assert_eq!(e.profile, Some("work".into()));
        }
    }

    #[test]
    fn test_edit_bad_usage() {
        assert!(parse_vec(&["edit"]).is_err());
        assert!(parse_vec(&["edit", "a", "b"]).is_err());
    }

    // ---------- VIEW ----------

    #[test]
    fn test_view_target_only() {
        if let Command::View(v) = parse_vec(&["view", "tmux"]).unwrap() {
            assert_eq!(v.target, "tmux");
            assert!(v.profile.is_none());
            assert!(!v.raw);
        }
    }

    #[test]
    fn test_view_target_only_raw() {
        if let Command::View(v) = parse_vec(&["view", "tmux", "--raw"]).unwrap() {
            assert_eq!(v.target, "tmux");
            assert!(v.profile.is_none());
            assert!(v.raw);
        }
    }

    #[test]
    fn test_view_profile() {
        if let Command::View(v) = parse_vec(&["view", "tmux/work"]).unwrap() {
            assert_eq!(v.target, "tmux");
            assert_eq!(v.profile, Some("work".into()));
            assert!(!v.raw);
        }
    }

    #[test]
    fn test_view_profile_raw() {
        if let Command::View(v) = parse_vec(&["view", "tmux/work", "--raw"]).unwrap() {
            assert_eq!(v.target, "tmux");
            assert_eq!(v.profile, Some("work".into()));
            assert!(v.raw);
        }
    }

    #[test]
    fn test_view_bad_usage() {
        assert!(parse_vec(&["view"]).is_err());
        assert!(parse_vec(&["view", "a", "b"]).is_err());
    }

    // ---------- LIST ----------

    #[test]
    fn test_list() {
        if let Command::List(l) = parse_vec(&["list"]).unwrap() {
            assert!(l.target.is_none());
        }
        if let Command::List(l) = parse_vec(&["list", "tmux"]).unwrap() {
            assert_eq!(l.target.unwrap(), "tmux");
        }
    }

    #[test]
    fn test_list_alias_ls() {
        if let Command::List(l) = parse_vec(&["ls"]).unwrap() {
            assert!(l.target.is_none());
        }
        if let Command::List(l) = parse_vec(&["ls", "tmux"]).unwrap() {
            assert_eq!(l.target.unwrap(), "tmux");
        }
    }

    // ---------- RM ----------

    #[test]
    fn test_rm_target() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux"]).unwrap() {
            assert_eq!(r.target, "tmux");
            assert!(r.profile.is_none());
        }
    }

    #[test]
    fn test_rm_profile() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux/work"]).unwrap() {
            assert_eq!(r.target, "tmux");
            assert_eq!(r.profile, Some("work".into()));
        }
    }

    // ---------- HELP / VERSION ----------

    #[test]
    fn test_help() {
        match parse_vec(&["add", "-h"]).unwrap() {
            Command::Help(t) if t == "add -h" => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_version() {
        match parse_vec(&["-v"]).unwrap() {
            Command::Version => (),
            _ => panic!(),
        }
    }
}
