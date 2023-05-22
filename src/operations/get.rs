use std::sync::{Arc, RwLock};

use crate::cache::Cache;
use crate::resp::parser::bulk_string::BulkString;
use crate::resp::redis_buffer::RedisBuffer;
use crate::resp::types::RedisError;
use crate::resp::Resp;

pub(super) fn process_get(
    array: Vec<Resp>,
    cache: &Arc<RwLock<Cache>>,
) -> Result<Vec<u8>, RedisError> {
    if array.len() != 2 {
        return Err(RedisError::Args("echo".to_string()));
    }
    let response = match array.get(1) {
        Some(Resp::BulkString { value: key }) => {
            BulkString::from(cache.read().unwrap().get(key.as_str()))
        }
        _ => {
            return Err(RedisError::Type);
        }
    };
    let mut send = vec![0; response.calc_len()];
    let _ = RedisBuffer::from((&response, send.as_mut()));
    Ok(send)
}
