pub mod error;
mod net;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
