#[derive(Debug)]
pub enum SuitError {
    Internal(String),
    BadUsage(String),
    Simple(String)

}

impl SuitError {
    pub fn message(msg: String) -> Self {
        Self::Simple(msg)
    }
}

impl std::fmt::Display for SuitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(err) => f.write_fmt(format_args!("suit: {}", err)),
            Self::BadUsage(err) => f.write_fmt(format_args!("suit: Wrong usage. {}\nUse `suit -h` for help.", err)),
            Self::Simple(msg)   => f.write_fmt(format_args!("suit: {}", msg)),
        }
    }
}

impl From<std::io::Error> for SuitError {
    fn from(value: std::io::Error) -> Self {
        Self::Internal(format!("IO error: {}", value))
    }
}

impl From<toml::de::Error> for SuitError {
    fn from(value: toml::de::Error) -> Self {
        Self::Internal(format!("Deserialization error: {}", value))
    }
}

impl From<toml::ser::Error> for SuitError {
    fn from(value: toml::ser::Error) -> Self {
        Self::Internal(format!("Serialization error: {}", value))
    }
}
