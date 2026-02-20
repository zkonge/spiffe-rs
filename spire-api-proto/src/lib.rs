pub mod agent;
mod macros;
pub mod server;
#[cfg(feature = "server")]
pub(crate) mod service;
mod types;

pub use self::types::*;

#[allow(dead_code)]
type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;
