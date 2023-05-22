pub mod cache;
pub mod operations;
pub mod resp;
pub mod startup;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
