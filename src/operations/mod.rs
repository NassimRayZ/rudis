mod echo;
mod get;
mod ping;
mod set;

use std::sync::{Arc, RwLock};

use crate::cache::{Cache, PriorityQueue};
use crate::resp::parser::{array::Array, error::Error};
use crate::resp::redis_buffer::RedisBuffer;
use crate::resp::types::RedisError;
use crate::resp::Resp;

use self::echo::process_echo;
use self::get::process_get;
use self::ping::process_ping;
use self::set::process_set;

enum Command {
    Ping,
    Echo,
    Set,
    Get,
}

pub fn process(buf: &mut [u8], cache: &Arc<RwLock<Cache>>, pq: &Arc<PriorityQueue>) -> Vec<u8> {
    let (array, cmd) = match retrieve_command(buf) {
        Ok(res) => res,
        Err(e) => return process_error(e),
    };
    let result = match cmd {
        Command::Ping => process_ping(array),
        Command::Echo => process_echo(array),
        Command::Set => process_set(array, cache, pq),
        Command::Get => process_get(array, cache),
    };
    match result {
        Ok(send) => send,
        Err(e) => process_error(e),
    }
}
fn process_error(e: RedisError) -> Vec<u8> {
    let error = Error::from(e);
    let mut send = vec![0; error.calc_len()];
    let _ = RedisBuffer::from((&error, send.as_mut()));
    send
}

fn retrieve_command(buf: &mut [u8]) -> Result<(Vec<Resp>, Command), RedisError> {
    let resp = Resp::try_from(buf)?;
    match resp {
        Resp::Array { value } => retreive_command_from_array(value),
        _ => Err(RedisError::Type.into()),
    }
}

fn retreive_command_from_array(mut array: Array) -> Result<(Vec<Resp>, Command), RedisError> {
    match array.first() {
        Some(Resp::BulkString { value }) => match value.as_str().to_lowercase().as_str() {
            "ping" => Ok((array.take().unwrap(), Command::Ping)),
            "echo" => Ok((array.take().unwrap(), Command::Echo)),
            "set" => Ok((array.take().unwrap(), Command::Set)),
            "get" => Ok((array.take().unwrap(), Command::Get)),
            _ => Err(RedisError::Unimplemented),
        },
        Some(_) => Err(RedisError::Unimplemented),
        None => Err(RedisError::Void),
    }
}
