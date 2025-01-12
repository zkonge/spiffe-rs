#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;
mod types;

#[cfg(feature = "client")]
pub use client::SpiffeWorkloadApiClient;
#[cfg(feature = "server")]
pub use server::{SpiffeWorkloadApi, SpiffeWorkloadApiServer};
pub use types::*;

#[cfg(any(feature = "client", feature = "server"))]
const SPIFFE_METADATA_KEY: &str = "workload.spiffe.io";
#[cfg(any(feature = "client", feature = "server"))]
const SPIFFE_METADATA_VALUE: &str = "true";
