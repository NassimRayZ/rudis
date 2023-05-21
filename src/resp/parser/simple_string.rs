use crate::resp::types::RedisError;

use super::{parse_string, Parser, RedisBuffer, CR, LF};

#[derive(Debug, PartialEq)]
pub struct SimpleString(String);

impl<'a> Parser<RedisBuffer<'a>> for SimpleString {
    type Output = Self;

    fn parse(buffer: &mut RedisBuffer<'a>) -> Result<Self::Output, RedisError> {
        let result = parse_string(buffer)?;
        Ok(Self(result))
    }
}

impl SimpleString {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn calc_len(&self) -> usize {
        self.0.len() + 3
    }
}
impl From<&str> for SimpleString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl ToString for SimpleString {
    fn to_string(&self) -> String {
        let (cr, lf) = (CR as char, LF as char);
        format!("+{}{}{}", self.0, cr, lf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_string_success() {
        let mut buf = vec![
            b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', CR, LF,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = SimpleString::parse(&mut buffer).unwrap_or(SimpleString("".into()));

        let expected = SimpleString("hello world".into());

        assert_eq!(result, expected);
    }
    #[test]
    fn parse_simple_string_failure() {
        let mut buf = vec![
            b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', CR,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = SimpleString::parse(&mut buffer).unwrap_or(SimpleString("".into()));

        let expected = SimpleString("".into());

        assert_eq!(result, expected);

        buf = vec![
            b'+', b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', LF,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);
        let result = SimpleString::parse(&mut buffer).unwrap_or(SimpleString("".into()));
        assert_eq!(result, expected);
    }
}
