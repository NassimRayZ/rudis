use super::{bulk_string::BulkString, error::Error, integer::Integer, simple_string::SimpleString};
use super::{parse_prefix, Parser, RedisBuffer, CR, LF};
use crate::resp::types::RedisError;
use crate::resp::Resp;

#[derive(Debug, PartialEq)]
pub struct Array(Option<Vec<Resp>>);

impl<'a> Parser<RedisBuffer<'a>> for Array {
    type Output = Self;

    fn parse(buffer: &mut RedisBuffer<'a>) -> Result<Self::Output, RedisError> {
        let size = match parse_prefix(buffer)? {
            Some(size) => size,
            None => return Ok(Self(None)),
        };
        let mut array = vec![];
        for _ in 0..size {
            array.push(match buffer.read_u8() {
                Some(b'+') => Resp::SimpleString {
                    value: SimpleString::parse(buffer)?,
                },
                Some(b'-') => Resp::Error {
                    value: Error::parse(buffer)?,
                },
                Some(b':') => Resp::Integer {
                    value: Integer::parse(buffer)?,
                },
                Some(b'$') => Resp::BulkString {
                    value: BulkString::parse(buffer)?,
                },
                Some(b'*') => Resp::Array {
                    value: Self::parse(buffer)?,
                },
                None => break,
                _ => {
                    return Err(RedisError::Type);
                }
            });
        }
        Ok(Array(Some(array)))
    }
}

// TODO: TURN `calc_len` INTO A TRAIT FUNCTION
//       THEN IMPLEMENT IT FOR ARRAY AND RESP
impl Array {
    pub fn get(&self) -> &Option<Vec<Resp>> {
        &self.0
    }
    pub fn first(&self) -> Option<&Resp> {
        match self.0 {
            Some(ref array) => array.first(),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.0 {
            Some(ref array) => array.is_empty(),
            None => true,
        }
    }
    pub fn len(&self) -> usize {
        match self.0 {
            Some(ref array) => array.len(),
            None => 0,
        }
    }
    pub fn take(&mut self) -> Option<Vec<Resp>> {
        std::mem::take(&mut self.0)
    }
}
impl ToString for Array {
    fn to_string(&self) -> String {
        let (cr, lf) = (CR as char, LF as char);
        match self.0 {
            Some(ref array) => {
                let mut result = vec![format!("*{}{}{}", array.len(), cr, lf)];

                for resp in array {
                    result.push(match resp {
                        Resp::SimpleString { value } => value.to_string(),
                        Resp::Error { value } => value.to_string(),
                        Resp::Integer { value } => value.to_string(),
                        Resp::BulkString { value } => value.to_string(),
                        Resp::Array { value } => value.to_string(),
                    });
                }
                result.concat()
            }
            None => format!("*-1{}{}", cr, lf),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::{CR, LF};
    use super::*;

    #[test]
    fn parse_array_success() {
        let mut buf = vec![
            b'2', CR, LF, b'$', b'5', CR, LF, b'h', b'e', b'l', b'l', b'o', CR, LF, b'$', b'5', CR,
            LF, b'w', b'o', b'r', b'l', b'd', CR, LF,
        ];

        let mut buffer = RedisBuffer::new(&mut buf);

        let result = match Array::parse(&mut buffer) {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                panic!()
            }
        };
        let expected = Array(Some(vec![
            Resp::BulkString {
                value: BulkString::new("hello"),
            },
            Resp::BulkString {
                value: BulkString::new("world"),
            },
        ]));

        assert_eq!(result, expected);
    }
    #[test]
    fn to_string_array_success() {
        let (cr, lf) = (CR as char, LF as char);
        let array = Array(Some(vec![]));
        let result = array.to_string();
        println!("{}", result);
        let expected = format!("*0{}{}", cr, lf);

        assert_eq!(result, expected);
    }
}
