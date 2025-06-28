#[cfg(feature = "low-level")]
pub mod proto;
#[cfg(not(feature = "low-level"))]
mod proto;
#[cfg(feature = "wrapper")]
mod wrapper;

#[cfg(any(feature = "client", feature = "server"))]
type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;

#[cfg(feature = "wrapper")]
pub use crate::wrapper::*;
