#[derive(Debug)]
pub enum RobeError {
    Internal(String),
    BadUsage(String),
    Hashing(String),
}

impl RobeError {
    pub fn message(msg: String) -> Self {
        Self::Internal(msg)
    }
    pub fn target_not_found(target: &str) -> Self {
        Self::message(format!("Target {} not found", target))
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
            Self::Hashing(err) => f.write_fmt(format_args!(
                "robe: An error occurred when hashing directory contents. {}",
                err
            )),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_internal() {
        let err = RobeError::Internal("error_message".to_string());
        let result = format!("{}", err);

        assert_eq!(result, "robe: error_message");
    }

    #[test]
    fn test_display_bad_usage() {
        let err = RobeError::BadUsage("error_message".to_string());
        let result = format!("{}", err);

        let expected = "robe: Wrong usage. error_message\nUse `robe -h` for help.";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_message_helper() {
        let err = RobeError::message("hello".to_string());

        match err {
            RobeError::Internal(msg) => assert_eq!(msg, "hello"),
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk exploded");

        let robe_err: RobeError = io_err.into();
        let msg = format!("{}", robe_err);

        assert!(msg.contains("IO error"));
        assert!(msg.contains("disk exploded"));
    }

    #[test]
    fn test_from_toml_deser_error() {
        let result: Result<u32, _> = toml::from_str("not_a_number");

        let err = result.unwrap_err();
        let robe_err: RobeError = err.into();

        let msg = format!("{}", robe_err);
        assert!(msg.contains("Deserialization error"));
    }
}
