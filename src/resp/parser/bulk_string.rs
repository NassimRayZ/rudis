use crate::resp::types::RedisError;

use super::{parse_prefix, parse_string, Parser, RedisBuffer, CR, LF};

#[derive(Debug, PartialEq)]
pub struct BulkString(Option<String>);

impl<'a> Parser<RedisBuffer<'a>> for BulkString {
    type Output = Self;

    fn parse(buffer: &mut RedisBuffer<'a>) -> Result<Self::Output, RedisError> {
        let size = match parse_prefix(buffer)? {
            Some(size) => size,
            None => return Ok(Self(None)),
        };
        let result = parse_string(buffer)?;
        if size != result.len() {
            return Err(RedisError::Size);
        }
        Ok(Self(Some(result)))
    }
}

impl BulkString {
    pub fn new(string: &str) -> Self {
        Self(Some(string.into()))
    }
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Some(string) => string.as_str(),
            None => panic!("None is not a string"),
        }
    }
    pub fn calc_len(&self) -> usize {
        match self.0 {
            Some(ref string) => {
                let len = string.len();

                let len_len = 'a: {
                    if len < 10 {
                        break 'a 1;
                    }
                    if len < 100 {
                        break 'a 2;
                    }
                    if len < 1000 {
                        break 'a 3;
                    }
                    if len < 10000 {
                        break 'a 4;
                    }
                    if len < 100000 {
                        break 'a 5;
                    }
                    if len < 1000000 {
                        break 'a 6;
                    }
                    if len < 10000000 {
                        break 'a 7;
                    }
                    if len < 100000000 {
                        break 'a 8;
                    }
                    if len < 1000000000 {
                        break 'a 9;
                    }
                    break 'a 0;
                };

                5 + len_len + len
            }
            None => 5,
        }
    }
}
impl From<&str> for BulkString {
    fn from(value: &str) -> Self {
        Self(Some(value.to_string()))
    }
}
impl ToString for BulkString {
    fn to_string(&self) -> String {
        let (cr, lf) = (CR as char, LF as char);
        match self.0 {
            Some(ref string) => format!("${}{}{}{}{}{}", string.len(), cr, lf, string, cr, lf),
            None => format!("$-1{}{}", cr, lf),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::{CR, LF};
    use super::*;

    #[test]
    fn parse_bulk_string_success() {
        let mut buf = vec![
            b'1', b'1', CR, LF, b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd',
            CR, LF,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = BulkString::parse(&mut buffer).unwrap();

        let expected = BulkString(Some("hello world".into()));

        assert_eq!(result, expected);
        let mut buf = vec![
            b'-', b'1', CR, LF, b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd',
            CR, LF,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);
        let result = BulkString::parse(&mut buffer).unwrap();
        let expected = BulkString(None);
        assert_eq!(result, expected);
    }
    #[test]
    fn parse_bulk_string_failure() {
        let mut buf = vec![
            b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', CR,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = BulkString::parse(&mut buffer).unwrap_or(BulkString(None));

        let expected = BulkString(None);

        assert_eq!(result, expected);

        let mut buf = vec![
            b'1', b'0', CR, LF, b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd',
            CR, LF,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);
        let result = BulkString::parse(&mut buffer).unwrap_or(BulkString(None));
        assert_eq!(result, expected);
    }
}
