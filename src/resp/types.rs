use std::fmt::Display;

#[derive(Debug)]
pub enum RedisError {
    Type,
    Parser,
    Buffer,
    Size,
    Void,
    Unimplemented,
    Args(String),
    UnknownOption(String),
}

impl Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            RedisError::Type => "ERROR: message does not correspond to any know type".to_string(),
            RedisError::Parser => "ERROR: failed to parse redis message".to_string(),
            RedisError::Buffer => "ERROR: failed to write into buffer".to_string(),
            RedisError::Size => {
                "ERROR: failed to parse redis message, actual and declared sizes do not match"
                    .to_string()
            }
            RedisError::Void => "ERROR: void message not allowed in this context".to_string(),
            RedisError::Unimplemented => {
                "ERROR: operation not allowed or unimplemented".to_string()
            }
            RedisError::Args(cmd) => {
                format!("ERROR: wrong number of arguments for '{}' command", cmd)
            }
            RedisError::UnknownOption(opt) => format!("ERROR: unknown option:  {}", opt),
        };

        write!(f, "{}", error)?;
        Ok(())
    }
}
impl std::error::Error for RedisError {}
