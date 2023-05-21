use crate::resp::types::RedisError;

use super::{parse_string, Parser, RedisBuffer, CR, LF};

#[derive(Debug, PartialEq)]
pub struct Error(String);

impl<'a> Parser<RedisBuffer<'a>> for Error {
    type Output = Self;

    fn parse(buffer: &mut RedisBuffer<'a>) -> Result<Self::Output, RedisError> {
        let result = parse_string(buffer)?;
        Ok(Self(result))
    }
}

impl Error {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn calc_len(&self) -> usize {
        self.0.len() + 3
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        let (cr, lf) = (CR as char, LF as char);
        format!("-{}{}{}", self.0, cr, lf)
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Self(value.to_string())
    }
}
