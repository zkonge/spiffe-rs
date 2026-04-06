mod client;
mod grpc;
mod server;
mod service;

pub use self::service::{NamedSvcFn, SvcFn, grpc_unimplemented};

pub type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;

// Re-export dependencies for use in macros via `$crate::`.
pub use futures_core;
pub use http;
pub use http_body;
pub use prost;
pub use tonic;
pub use tonic_prost;
pub use tower_service;
