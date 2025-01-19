#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
mod types;

pub use types::*;

#[cfg(any(feature = "client", feature = "server"))]
const SPIFFE_METADATA_KEY: &str = "workload.spiffe.io";
#[cfg(any(feature = "client", feature = "server"))]
const SPIFFE_METADATA_VALUE: &str = "true";
