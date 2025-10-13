pub mod proto;
#[cfg(feature = "wrapper")]
mod wrapper;

#[cfg(any(feature = "client", feature = "server"))]
type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;

#[cfg(feature = "wrapper")]
pub use crate::wrapper::*;
pub use spiffe_id;
