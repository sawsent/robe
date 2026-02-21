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
        "add" if args.contains(&"-r".to_string()) || args.contains(&"--register".to_string()) => {
            Register::parse(args)
        }
        "add" => Add::parse(args),
        "edit" => Edit::parse(args),
        "use" => Use::parse(args),
        "list" => List::parse(args, "list"),
        "ls" => List::parse(args, "ls"),
        "rm" => Rm::parse(args),
        "view" => View::parse(args),
        "status" => Status::parse(args),
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
    Register(Register),
    Edit(Edit),
    Use(Use),
    View(View),
    List(List),
    Rm(Rm),
    Status(Status),
}

#[derive(Debug, Clone, Default)]
pub struct Add {
    pub target: String,
    pub profile: String,
    pub force: bool,
}

impl Add {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe add <target>/<profile> [-r file] [-f]".to_string())
    }

    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.is_empty() {
            return Err(Self::bu());
        }

        let mut cmd = Self::default();
        let mut i = 0;
        let mut seen_target = false;

        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-f" | "--force" => cmd.force = true,
                t if !seen_target => {
                    let (target, profile) = split_target_and_profile(t, Self::bu)?;
                    cmd.target = target;
                    cmd.profile = profile;
                    seen_target = true;
                }
                _ => return Err(Self::bu()),
            }
            i += 1;
        }

        if seen_target {
            Ok(Command::Add(cmd))
        } else {
            Err(Self::bu())
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Register {
    pub target: String,
    pub profile: String,
    pub register_file_path: PathBuf,
}

impl Register {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe add <target>/<profile> [-r <path>]".to_string())
    }

    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        let mut cmd = Register::default();
        let mut i = 0;
        let mut seen_target = false;

        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-r" | "--register" => {
                    i += 1;
                    if let Some(f) = args.get(i) {
                        cmd.register_file_path = PathBuf::from(&f);
                    } else {
                        return Err(Self::bu());
                    }
                }
                t if !seen_target => {
                    let (target, profile) = split_target_and_profile(t, Self::bu)?;
                    cmd.target = target;
                    cmd.profile = profile;
                    seen_target = true;
                }
                _ => return Err(Self::bu()),
            }
            i += 1;
        }

        if seen_target {
            Ok(Command::Register(cmd))
        } else {
            Err(Self::bu())
        }
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
        if args.len() != 1 {
            return Err(Self::bu());
        }

        let first = args[0].clone();

        let (target, profile) = if first.contains('/') {
            let (t, p) = split_target_and_profile(&first, Self::bu)?;
            (t, Some(p))
        } else {
            (first, None)
        };

        Ok(Command::Edit(Self { target, profile }))
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
        let mut i = 0;
        let mut seen_target = false;
        let mut cmd = Self::default();

        while let Some(arg) = args.get(i) {
            match arg.as_str() {
                "--raw" => cmd.raw = true,
                tp if !seen_target => {
                    let (target, profile) = if tp.contains('/') {
                        let (t, p) = split_target_and_profile(tp, Self::bu)?;
                        (t, Some(p))
                    } else {
                        (tp.to_string(), None)
                    };
                    cmd.target = target;
                    cmd.profile = profile;
                    seen_target = true;
                }
                _ => return Err(Self::bu()),
            }
            i += 1;
        }

        if seen_target {
            Ok(Command::View(cmd))
        } else {
            Err(Self::bu())
        }
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
            Ok(Command::Use(Self { target, profile }))
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
            Ok(Command::List(Self { target }))
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

        Ok(Command::Rm(Self { target, profile }))
    }
}

#[derive(Debug, Clone)]
pub struct Status {
    pub target: Option<String>,
}

impl Status {
    fn bu() -> RobeError {
        RobeError::BadUsage("Usage: robe status [<target>]".to_string())
    }
    pub fn parse(args: &[String]) -> Result<Command, RobeError> {
        if args.len() > 1 {
            Err(Self::bu())
        } else {
            let target = args.first().cloned();
            Ok(Command::Status(Self { target }))
        }
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
    fn test_add() {
        match parse_vec(&["add", "target/profile"]).unwrap() {
            Command::Add(a) => {
                assert_eq!(a.target, "target");
                assert_eq!(a.profile, "profile");
                assert!(!a.force);
            }
            _ => panic!("Expected Add"),
        }
    }

    #[test]
    fn test_add_force() {
        match parse_vec(&["add", "target/profile", "-f"]).unwrap() {
            Command::Add(a) => {
                assert!(a.force);
            }
            _ => panic!("Expected Add"),
        }
        match parse_vec(&["add", "-f", "target/profile"]).unwrap() {
            Command::Add(a) => {
                assert!(a.force);
            }
            _ => panic!("Expected Add"),
        }
    }

    #[test]
    fn test_add_bad_usage() {
        match parse_vec(&["add", "target/profile", "-f", "other/other"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["add"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["add", "-f"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    // ---------- REGISTER ----------

    #[test]
    fn test_register() {
        match parse_vec(&["add", "target/profile", "-r", "filename"]).unwrap() {
            Command::Register(a) => {
                assert_eq!(a.target, "target");
                assert_eq!(a.profile, "profile");
                assert_eq!(a.register_file_path, PathBuf::from("filename"));
            }
            _ => panic!("Expected Register"),
        }
        match parse_vec(&["add", "--register", "file", "target/profile"]).unwrap() {
            Command::Register(a) => {
                assert_eq!(a.target, "target");
                assert_eq!(a.profile, "profile");
                assert_eq!(a.register_file_path, PathBuf::from("file"));
            }
            _ => panic!("Expected Register"),
        }
    }

    #[test]
    fn test_register_bad_usage() {
        match parse_vec(&["add", "target/profile", "other/other", "-r", "filename"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&[
            "add",
            "target/profile",
            "other/other",
            "-r",
            "filename",
            "filename2",
        ])
        .unwrap_err()
        {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["add", "target/profile", "-r", "filename", "-f"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("add <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    // ---------- USE ----------

    #[test]
    fn test_use() {
        match parse_vec(&["use", "target/profile"]).unwrap() {
            Command::Use(u) => {
                assert_eq!(u.target, "target");
                assert_eq!(u.profile, "profile");
            }
            _ => panic!("Expected Use"),
        }
        if let Command::Use(u) = parse_vec(&["use", "target/profile"]).unwrap() {
            assert_eq!(u.target, "target");
            assert_eq!(u.profile, "profile");
        }
    }

    #[test]
    fn test_use_bad_usage() {
        match parse_vec(&["use"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe use <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["use", "a", "b"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe use <target>/<profile>"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    // ---------- EDIT ----------

    #[test]
    fn test_edit_target_only() {
        match parse_vec(&["edit", "target"]).unwrap() {
            Command::Edit(e) => {
                assert_eq!(e.target, "target");
                assert!(e.profile.is_none());
            }
            _ => panic!("Expected Edit"),
        }
    }

    #[test]
    fn test_edit_profile() {
        match parse_vec(&["edit", "target/profile"]).unwrap() {
            Command::Edit(e) => {
                assert_eq!(e.target, "target");
                assert_eq!(e.profile, Some("profile".into()));
            }
            _ => panic!("Expected Edit"),
        }
    }

    #[test]
    fn test_edit_bad_usage() {
        match parse_vec(&["edit"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe edit <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["edit", "a", "b"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe edit <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    // ---------- VIEW ----------

    #[test]
    fn test_view_target_only() {
        match parse_vec(&["view", "target"]).unwrap() {
            Command::View(v) => {
                assert_eq!(v.target, "target");
                assert!(v.profile.is_none());
                assert!(!v.raw);
            }
            _ => panic!("Expected View"),
        }
    }

    #[test]
    fn test_view_target_only_raw() {
        match parse_vec(&["view", "target", "--raw"]).unwrap() {
            Command::View(v) => {
                assert_eq!(v.target, "target");
                assert!(v.profile.is_none());
                assert!(v.raw);
            }
            _ => panic!("Expected View"),
        }
    }

    #[test]
    fn test_view_profile() {
        match parse_vec(&["view", "target/profile"]).unwrap() {
            Command::View(v) => {
                assert_eq!(v.target, "target");
                assert_eq!(v.profile, Some("profile".into()));
                assert!(!v.raw);
            }
            _ => panic!("Expected View"),
        }
    }

    #[test]
    fn test_view_profile_raw() {
        match parse_vec(&["view", "target/profile", "--raw"]).unwrap() {
            Command::View(v) => {
                assert_eq!(v.target, "target");
                assert_eq!(v.profile, Some("profile".into()));
                assert!(v.raw);
            }
            _ => panic!("Expected View"),
        }
        match parse_vec(&["view", "--raw", "target/profile"]).unwrap() {
            Command::View(v) => {
                assert_eq!(v.target, "target");
                assert_eq!(v.profile, Some("profile".into()));
                assert!(v.raw);
            }
            _ => panic!("Expected View"),
        }
    }

    #[test]
    fn test_view_bad_usage() {
        match parse_vec(&["view"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe view <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["view", "a", "b"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe view <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["view", "--raw"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe view <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    // ---------- LIST ----------

    #[test]
    fn test_list_bad_usage() {
        match parse_vec(&["list", "bad", "usage"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe list [<target>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
        match parse_vec(&["ls", "bad", "usage"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe ls [<target>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    #[test]
    fn test_list() {
        match parse_vec(&["list"]).unwrap() {
            Command::List(l) => {
                assert!(l.target.is_none());
            }
            _ => panic!("Expected List"),
        }
        match parse_vec(&["list", "target"]).unwrap() {
            Command::List(l) => {
                assert_eq!(l.target.unwrap(), "target");
            }
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn test_list_alias_ls() {
        match parse_vec(&["ls"]).unwrap() {
            Command::List(l) => {
                assert!(l.target.is_none());
            }
            _ => panic!("Expected List"),
        }
        match parse_vec(&["ls", "target"]).unwrap() {
            Command::List(l) => {
                assert_eq!(l.target.unwrap(), "target");
            }
            _ => panic!("Expected List"),
        }
    }

    // ---------- RM ----------

    #[test]
    fn test_rm_bad_usage() {
        match parse_vec(&["rm"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("robe rm <target>[/<profile>]"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    #[test]
    fn test_rm_target() {
        match parse_vec(&["rm", "target"]).unwrap() {
            Command::Rm(r) => {
                assert_eq!(r.target, "target");
                assert!(r.profile.is_none());
            }
            _ => panic!("Expected Rm"),
        }
    }

    #[test]
    fn test_rm_profile() {
        match parse_vec(&["rm", "target/profile"]).unwrap() {
            Command::Rm(r) => {
                assert_eq!(r.target, "target");
                assert_eq!(r.profile, Some("profile".into()));
            }
            _ => panic!("Expected Rm"),
        }
    }

    // ---------- HELP / VERSION ----------

    #[test]
    fn test_help() {
        match parse_vec(&["add", "-h"]).unwrap() {
            Command::Help(t) if t == "add -h" => (),
            _ => panic!("Expected Help"),
        }
        match parse_vec(&["add", "--help"]).unwrap() {
            Command::Help(t) if t == "add --help" => (),
            _ => panic!("Expected Help"),
        }
    }

    #[test]
    fn test_version() {
        match parse_vec(&["-v"]).unwrap() {
            Command::Version => (),
            _ => panic!("Expected Version"),
        }
        match parse_vec(&["--version"]).unwrap() {
            Command::Version => (),
            _ => panic!("Expected Version"),
        }
    }

    // ---------- UNRECOGNIZED / NOT PROVIDED ----------

    #[test]
    fn test_graceful_handle_unrecognized_command() {
        match parse_vec(&["unrecognized"]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("Command not recognized"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }

    #[test]
    fn test_graceful_handle_not_provided_command() {
        match parse_vec(&[]).unwrap_err() {
            RobeError::BadUsage(msg) => {
                assert!(msg.contains("No command provided"));
            }
            _ => panic!("Expected BadUsage"),
        }
    }
}
