use crate::resp::types::RedisError;

use super::{parse_string, Parser, RedisBuffer, CR, LF};

#[derive(Debug, PartialEq)]
pub struct Integer(i64);

impl<'a> Parser<RedisBuffer<'a>> for Integer {
    type Output = Self;

    fn parse(buffer: &mut RedisBuffer<'a>) -> Result<Self::Output, RedisError> {
        let int_string = parse_string(buffer)?;
        let result: i64 = match int_string.parse() {
            Ok(int) => int,
            Err(_) => return Err(RedisError::Parser),
        };

        Ok(Self(result))
    }
}

impl Integer {
    pub fn calc_len(&self) -> usize {
        let int = self.0;
        let len = 'a: {
            if int < 10 {
                break 'a 1;
            }
            if int < 100 {
                break 'a 2;
            }
            if int < 1000 {
                break 'a 3;
            }
            if int < 10000 {
                break 'a 4;
            }
            if int < 100000 {
                break 'a 5;
            }
            if int < 1000000 {
                break 'a 6;
            }
            if int < 10000000 {
                break 'a 7;
            }
            if int < 100000000 {
                break 'a 8;
            }
            if int < 1000000000 {
                break 'a 9;
            }
            break 'a 0;
        };
        len + 3
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        let (cr, lf) = (CR as char, LF as char);
        format!(":{}{}{}", self.0, cr, lf)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CR, LF};
    use super::*;

    #[test]
    fn parse_integer_success() {
        let mut buf = vec![b'-', b'1', b'1', b'3', CR, LF];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = Integer::parse(&mut buffer).unwrap();

        let expected = Integer(-113);

        assert_eq!(result, expected);
    }
    #[test]
    fn parse_integer_failure() {
        let mut buf = vec![
            b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', CR,
        ];
        let mut buffer = RedisBuffer::new(&mut buf);

        let result = Integer::parse(&mut buffer).unwrap_or(Integer(-1));

        let expected = Integer(-1);

        assert_eq!(result, expected);
    }
}
