pub mod agent;
mod macros;
#[cfg(feature = "server")]
pub(crate) mod service;
mod types;

pub use self::types::*;

type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;
