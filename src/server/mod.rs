pub mod server;
mod config;
pub mod handler;
mod packet;
mod conn;
mod error;
mod codec;
mod cmd;


use std::sync::{Arc, RwLock};
pub use self::config::{Config};
pub use self::error::IoError;
pub use self::handler::CmppHandler;
pub use self::handler::{CmppLoginHandler, Cmpp3SubmitHandler};
pub use self::conn::{Conn};
pub use self::codec::{CmppMessage, CmppDecoder, CmppEncoder};

pub type Handlers = Vec<Arc<RwLock<dyn CmppHandler>>>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;