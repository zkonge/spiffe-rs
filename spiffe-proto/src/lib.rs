#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
mod types;

pub use types::*;

type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;

const SPIFFE_METADATA_KEY: &str = "workload.spiffe.io";
const SPIFFE_METADATA_VALUE: &str = "true";
