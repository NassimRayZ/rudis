pub mod array;
pub mod bulk_string;
pub mod error;
pub mod integer;
pub mod simple_string;

pub const CR: u8 = b'\r';
pub const LF: u8 = b'\n';
const ASCII_START: u8 = b'0';
use crate::resp::types::RedisError;

pub(self) use super::redis_buffer::RedisBuffer;

pub trait Parser<IN> {
    type Output;
    fn parse(input: &mut IN) -> Result<Self::Output, RedisError>;
}

pub fn process_eof(buffer: &mut RedisBuffer) -> Result<(), RedisError> {
    match buffer.read_u8() {
        Some(&LF) => Ok(()),
        _ => Err(RedisError::Parser),
    }
}
pub fn parse_string(buffer: &mut RedisBuffer) -> Result<String, RedisError> {
    let mut result = String::new();
    loop {
        match buffer.read_u8() {
            Some(&CR) => {
                process_eof(buffer)?;
                break;
            }
            Some(byte) => result.push(*byte as char),
            None => return Err(RedisError::Parser),
        }
    }
    Ok(result)
}

pub fn parse_prefix(buffer: &mut RedisBuffer) -> Result<Option<usize>, RedisError> {
    let lhs = *buffer.read_u8().unwrap();
    let rhs = *buffer.read_u8().unwrap();
    let size = match (lhs, rhs) {
        (b'-', b'1') => return Ok(None),
        (b'-', _) => return Err(RedisError::Parser),
        (_, CR) => {
            process_eof(buffer)?;
            (lhs - ASCII_START) as usize
        }
        (_, _) => {
            let lhs = lhs - ASCII_START;
            let rhs = rhs - ASCII_START;
            let size_str = format!("{}{}{}", lhs, rhs, &parse_string(buffer)?);
            match size_str.parse::<usize>() {
                Ok(size) => size,
                Err(_) => return Err(RedisError::Parser),
            }
        }
    };
    Ok(Some(size))
}
