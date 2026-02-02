#[derive(Debug)]
pub enum RobeError {
    Internal(String),
    BadUsage(String),
    Simple(String),
}

impl RobeError {
    pub fn message(msg: String) -> Self {
        Self::Simple(msg)
    }
}

impl std::fmt::Display for RobeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(err) => f.write_fmt(format_args!("robe: {}", err)),
            Self::BadUsage(err) => f.write_fmt(format_args!(
                "robe: Wrong usage. {}\nUse `robe -h` for help.",
                err
            )),
            Self::Simple(msg) => f.write_fmt(format_args!("robe: {}", msg)),
        }
    }
}

impl From<std::io::Error> for RobeError {
    fn from(value: std::io::Error) -> Self {
        Self::Internal(format!("IO error: {}", value))
    }
}

impl From<toml::de::Error> for RobeError {
    fn from(value: toml::de::Error) -> Self {
        Self::Internal(format!("Deserialization error: {}", value))
    }
}

impl From<toml::ser::Error> for RobeError {
    fn from(value: toml::ser::Error) -> Self {
        Self::Internal(format!("Serialization error: {}", value))
    }
}
