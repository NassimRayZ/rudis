pub mod parser;
pub mod redis_buffer;
pub mod types;

use types::RedisError;

use parser::Parser;
use parser::{
    array::Array, bulk_string::BulkString, error::Error, integer::Integer,
    simple_string::SimpleString,
};
use redis_buffer::RedisBuffer;

//
#[derive(Debug, PartialEq)]
pub enum Resp {
    SimpleString { value: SimpleString },
    Error { value: Error },
    Integer { value: Integer },
    BulkString { value: BulkString },
    Array { value: Array },
}

impl TryFrom<&mut [u8]> for Resp {
    type Error = RedisError;

    fn try_from(mut buf: &mut [u8]) -> Result<Self, <Self as TryFrom<&mut [u8]>>::Error> {
        let mut buffer = RedisBuffer::new(&mut buf);
        match buffer.read_u8() {
            Some(b'+') => Ok(Self::SimpleString {
                value: SimpleString::parse(&mut buffer)?,
            }),
            Some(b'-') => Ok(Self::Error {
                value: Error::parse(&mut buffer)?,
            }),
            Some(b':') => Ok(Self::Integer {
                value: Integer::parse(&mut buffer)?,
            }),
            Some(b'$') => Ok(Self::BulkString {
                value: BulkString::parse(&mut buffer)?,
            }),
            Some(b'*') => Ok(Self::Array {
                value: Array::parse(&mut buffer)?,
            }),
            _ => Err(RedisError::Type),
        }
    }
}
