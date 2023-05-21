pub mod handler;
pub mod operations;
pub mod resp;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
