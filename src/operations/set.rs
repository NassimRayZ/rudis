use std::sync::{Arc, RwLock};

use crate::cache::Cache;
use crate::resp::parser::simple_string::SimpleString;
use crate::resp::redis_buffer::RedisBuffer;
use crate::resp::types::RedisError;
use crate::resp::Resp;

pub(super) fn process_set(
    array: Vec<Resp>,
    cache: &Arc<RwLock<Cache>>,
) -> Result<Vec<u8>, RedisError> {
    if array.len() != 3 {
        return Err(RedisError::Args("echo".to_string()));
    }

    match (array.get(1), array.get(2)) {
        (Some(Resp::BulkString { value: key }), Some(Resp::BulkString { value })) => {
            cache.write().unwrap().set(key.as_str(), value.as_str())
        }
        (_, _) => {
            return Err(RedisError::Type);
        }
    };
    let response = SimpleString::from("OK");
    let mut send = vec![0; response.calc_len()];
    let _ = RedisBuffer::from((&response, send.as_mut()));
    Ok(send)
}
