use crate::resp::{
    parser::simple_string::SimpleString, redis_buffer::RedisBuffer, types::RedisError, Resp,
};

pub fn process_ping(array: Vec<Resp>) -> Result<Vec<u8>, RedisError> {
    let mut send: Vec<u8>;
    if array.len() > 2 {
        return Err(RedisError::Args("ping".into()));
    }
    if array.len() == 1 {
        let response = SimpleString::from("PONG");
        send = vec![0; response.calc_len()];
        let _ = RedisBuffer::from((&response, send.as_mut()));
        Ok(send)
    } else {
        let response = match array.get(1) {
            Some(Resp::BulkString { value }) => value,
            _ => {
                return Err(RedisError::Type);
            }
        };
        send = vec![0; response.calc_len()];
        let _ = RedisBuffer::from((response, send.as_mut()));
        Ok(send)
    }
}
