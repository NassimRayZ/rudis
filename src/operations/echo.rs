use crate::resp::{redis_buffer::RedisBuffer, types::RedisError, Resp};

pub(super) fn process_echo(array: Vec<Resp>) -> Result<Vec<u8>, RedisError> {
    if array.len() == 1 {
        return Err(RedisError::Args("echo".to_string()));
    }

    let response = match array.get(1) {
        Some(Resp::BulkString { value }) => value,
        _ => {
            return Err(RedisError::Type);
        }
    };
    let mut send = vec![0; response.calc_len()];
    let _ = RedisBuffer::from((response, send.as_mut()));
    Ok(send)
}
