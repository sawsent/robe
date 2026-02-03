use crate::errors::RobeError;
use std::path::PathBuf;

pub fn parse_cmd(args: &Vec<String>) -> Result<Command, RobeError> {
    for arg in args {
        if arg == "-h" || arg == "--help" {
            return Ok(Command::Help(args.join(" ")));
        } else if arg == "-v" || arg == "--version" {
            return Ok(Command::Version);
        }
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
        "use" => Use::parse(args),
        "list" => List::parse(args),
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

        // First argument: target/profile
        if let Some(j) = args.get(i) {
            let (target, profile) = split_target_and_profile(j, Add::bu)?;
            cmd.target = target;
            cmd.profile = profile;
            i += 1;
        } else {
            return Err(Add::bu());
        }

        // Optional flags
        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-r" | "--register" => {
                    i += 1;
                    if let Some(f) = args.get(i) {
                        cmd.to_register = Some(PathBuf::from(&f).canonicalize()?);
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
pub struct View {
    pub target: String,
    pub profile: Option<String>,
}

impl View {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe view <target>[/<profile>]".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() {
            return Err(Self::bu());
        }

        let first = args[0].clone();

        // Check if profile is included
        let (target, profile) = if first.contains('/') {
            let (t, p) = split_target_and_profile(&first, Self::bu)?;
            (t, Some(p))
        } else {
            (first, None)
        };

        Ok(Command::View(View { target, profile }))
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
        if let Some(j) = args.first() {
            let (target, profile) = split_target_and_profile(j, Self::bu)?;
            Ok(Command::Use(Use { target, profile }))
        } else {
            Err(Self::bu())
        }
    }
}

#[derive(Debug, Clone)]
pub struct List {
    pub target: Option<String>,
}

impl List {
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        let target = args.first().cloned();
        Ok(Command::List(List { target }))
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
        if args.is_empty() {
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
        parse_cmd(&args.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn test_add_basic() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work"]).unwrap() {
            assert_eq!(a.target, "tmux");
            assert_eq!(a.profile, "work");
            assert!(a.to_register.is_none());
            assert!(!a.force);
        } else {
            panic!("Expected Add command");
        }
    }

    #[test]
    fn test_add_force() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work", "-f"]).unwrap() {
            assert_eq!(a.to_register, None);
            assert!(a.force);
        }
    }

    #[test]
    fn test_use() {
        if let Command::Use(u) = parse_vec(&["use", "tmux/work"]).unwrap() {
            assert_eq!(u.target, "tmux");
            assert_eq!(u.profile, "work");
        }
    }

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
    fn test_rm_target() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux"]).unwrap() {
            assert_eq!(r.target, "tmux");
            assert_eq!(r.profile, None);
        }
    }

    #[test]
    fn test_rm_profile() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux/work"]).unwrap() {
            assert_eq!(r.target, "tmux");
            assert_eq!(r.profile, Some("work".to_string()));
        }
    }

    #[test]
    fn test_help() {
        match parse_vec(&["restore", "tmux", "-h"]).unwrap() {
            Command::Help(t) if t == "restore tmux -h" => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_version() {
        match parse_vec(&["-v", "add", "tmux"]).unwrap() {
            Command::Version => (),
            _ => panic!(),
        }
    }
}
