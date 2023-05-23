use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::cache::{Cache, PriorityQueue, State};
use crate::resp::parser::simple_string::SimpleString;
use crate::resp::redis_buffer::RedisBuffer;
use crate::resp::types::RedisError;
use crate::resp::Resp;

pub(super) fn process_set(
    array: Vec<Resp>,
    cache: &Arc<RwLock<Cache>>,
    pq: &Arc<PriorityQueue>,
) -> Result<Vec<u8>, RedisError> {
    if array.len() != 3 || array.len() != 5 {}
    match array.len() {
        3 => match (array.get(1).unwrap(), array.get(2).unwrap()) {
            (Resp::BulkString { value: key }, Resp::BulkString { value }) => {
                cache
                    .write()
                    .expect("Failed to aquire `RwLock` to write")
                    .set(key.as_str(), value.as_str());
            }
            (_, _) => {
                return Err(RedisError::Type);
            }
        },
        5 => process_set_opt(array, cache, pq)?,
        _ => return Err(RedisError::Args("set".to_string())),
    };
    let response = SimpleString::from("OK");
    let mut send = vec![0; response.calc_len()];
    let _ = RedisBuffer::from((&response, send.as_mut()));
    Ok(send)
}

fn process_set_opt(
    array: Vec<Resp>,
    cache: &Arc<RwLock<Cache>>,
    pq: &Arc<PriorityQueue>,
) -> Result<(), RedisError> {
    let key = array.get(1).unwrap();
    let value = array.get(2).unwrap();
    let opt = array.get(3).unwrap();
    let expiry = array.get(4).unwrap();

    match (key, value, opt, expiry) {
        (
            Resp::BulkString { value: key },
            Resp::BulkString { value },
            Resp::BulkString { value: opt },
            Resp::BulkString { value: expiry },
        ) => {
            if opt.as_str().to_uppercase().as_str() != "EX" {
                return Err(RedisError::UnknownOption(opt.as_str().to_string()));
            }
            let expiry: u64 = expiry
                .as_str()
                .parse()
                .expect(&format!("Failed to parse expiry {}", expiry.as_str()));
            let instant = Instant::now() + Duration::from_secs(expiry);
            cache
                .write()
                .expect("Failed to aquire `RwLock` to write")
                .set(key.as_str(), value.as_str());
            pq.entries
                .lock()
                .expect("Failed to aquire `Mutex` from sender")
                .push(State::new(key.as_str(), instant));

            pq.filled.notify_one();
        }
        (_, _, _, _) => return Err(RedisError::Type),
    }

    Ok(())
}
