pub mod server;
mod config;

mod conn;
mod error;
mod codec;
mod cmd;
mod handler;

pub use self::config::{Config};
pub use self::error::IoError;
pub use self::conn::{Conn};
pub use self::codec::{CmppMessage, CmppDecoder, CmppEncoder};


pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;