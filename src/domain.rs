use crate::errors::SuitError;
use std::path::PathBuf;

pub fn parse_cmd(args: &Vec<String>) -> Result<Command, SuitError> {
    for arg in args {
        if arg == "-h" || arg == "--help" {
            return Ok(Command::Help(args.join(" ")));
        } else if arg == "-v" || arg == "--version" {
            return Ok(Command::Version);
        }
    }
    if let Some(cmd) = args.get(0) {
        parse_internal(&cmd, &args.get(1..).unwrap_or(&[]).to_vec())
    } else {
        Err(SuitError::BadUsage("No command provided.".to_string()))
    }
}

fn parse_internal(cmd: &str, args: &Vec<String>) -> Result<Command, SuitError> {
    match cmd {
        "add" => Add::parse(args),
        "use" => Use::parse(args),
        "list" => List::parse(args),
        "rm" => Rm::parse(args),
        other => Err(SuitError::BadUsage(format!("Command not recognized: {}", other))),
    }
}

fn split_tool_and_profile<F>(joined: &str, bad_usage: F) -> Result<(String, String), SuitError>
where
    F: Fn() -> SuitError,
{
    let split: Vec<&str> = joined.split('/').collect();
    if split.len() == 2 {
        if let (Some(t), Some(p)) = (split.get(0), split.get(1)) {
            return Ok((t.to_string(), p.to_string()));
        }
    }
    Err(bad_usage())
}

#[derive(Debug, Clone)]
pub enum Command {
    Help(String),
    Version,
    Add(Add),
    Use(Use),
    List(List),
    Rm(Rm),
}

#[derive(Debug, Clone, Default)]
pub struct Add {
    pub tool: String,
    pub profile: String,
    pub to_register: Option<PathBuf>,
    pub force: bool,
}

impl Add {
    fn bad_usage() -> SuitError {
        SuitError::BadUsage("Usage: suit add <tool>/<profile> [-r file] [-f]".to_string())
    }

    pub fn parse(args: &Vec<String>) -> Result<Command, SuitError> {
        let mut cmd = Add::default();
        let mut i = 0;

        // First argument: tool/profile
        if let Some(j) = args.get(i) {
            let (tool, profile) = split_tool_and_profile(j, Add::bad_usage)?;
            cmd.tool = tool;
            cmd.profile = profile;
            i += 1;
        } else {
            return Err(Add::bad_usage());
        }

        // Optional flags
        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-r" | "--register" => {
                    i += 1;
                    if let Some(f) = args.get(i) {
                        cmd.to_register = Some(PathBuf::from(&f).canonicalize()?);
                    } else {
                        return Err(Add::bad_usage());
                    }
                }
                "-f" | "--force" => cmd.force = true,
                _ => return Err(Add::bad_usage()),
            }
            i += 1;
        }

        Ok(Command::Add(cmd))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Use {
    pub tool: String,
    pub profile: String,
}

impl Use {
    pub fn parse(args: &Vec<String>) -> Result<Command, SuitError> {
        if let Some(j) = args.get(0) {
            let (tool, profile) = split_tool_and_profile(j, || {
                SuitError::BadUsage("Usage: suit use <tool>/<profile>".to_string())
            })?;
            Ok(Command::Use(Use { tool, profile }))
        } else {
            Err(SuitError::BadUsage("Usage: suit use <tool>/<profile>".to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct List {
    pub tool: Option<String>,
}

impl List {
    pub fn parse(args: &Vec<String>) -> Result<Command, SuitError> {
        let tool = args.get(0).cloned();
        Ok(Command::List(List { tool }))
    }
}

#[derive(Debug, Clone)]
pub struct Rm {
    pub tool: String,
    pub profile: Option<String>
}

impl Rm {
    pub fn parse(args: &Vec<String>) -> Result<Command, SuitError> {
        if args.is_empty() {
            return Err(SuitError::BadUsage(
                "Usage: suit rm <tool>[/<profile>]".to_string(),
            ));
        }

        let first = args[0].clone();


        // Check if profile is included
        let (tool, profile) = if first.contains('/') {
            let (t, p) = split_tool_and_profile(&first, || {
                SuitError::BadUsage("Usage: suit rm <tool>[/<profile>] [-f]".to_string())
            })?;
            (t, Some(p))
        } else {
            (first, None)
        };

        Ok(Command::Rm(Rm {
            tool,
            profile,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_vec(args: &[&str]) -> Result<Command, SuitError> {
        parse_cmd(&args.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn test_add_basic() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work"]).unwrap() {
            assert_eq!(a.tool, "tmux");
            assert_eq!(a.profile, "work");
            assert!(a.to_register.is_none());
            assert!(!a.force);
        } else {
            panic!("Expected Add command");
        }
    }

    #[test]
    fn test_add_force() {
        if let Command::Add(a) = parse_vec(&["add", "tmux/work", "-f"]).unwrap()
        {
            assert_eq!(a.to_register, None);
            assert!(a.force);
        }
    }

    #[test]
    fn test_use() {
        if let Command::Use(u) = parse_vec(&["use", "tmux/work"]).unwrap() {
            assert_eq!(u.tool, "tmux");
            assert_eq!(u.profile, "work");
        }
    }

    #[test]
    fn test_list() {
        if let Command::List(l) = parse_vec(&["list"]).unwrap() {
            assert!(l.tool.is_none());
        }
        if let Command::List(l) = parse_vec(&["list", "tmux"]).unwrap() {
            assert_eq!(l.tool.unwrap(), "tmux");
        }
    }

    #[test]
    fn test_rm_tool() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux"]).unwrap() {
            assert_eq!(r.tool, "tmux");
            assert_eq!(r.profile, None);
        }
    }

    #[test]
    fn test_rm_profile() {
        if let Command::Rm(r) = parse_vec(&["rm", "tmux/work"]).unwrap() {
            assert_eq!(r.tool, "tmux");
            assert_eq!(r.profile, Some("work".to_string()));
        }
    }

    #[test]
    fn test_help() {
        match parse_vec(&["restore", "tmux", "-h"]).unwrap() {
            Command::Help(t) if t == "restore tmux -h" => (),
            _ => panic!()
        }
    }

    #[test]
    fn test_version() {
        match parse_vec(&["-v", "add", "tmux"]).unwrap() {
            Command::Version => (),
            _ => panic!()
        }
    }
}

