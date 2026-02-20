#![allow(unused_imports, unused_macros)]

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub(crate) use self::client::define_client;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub(crate) use self::server::define_server;

#[cfg(any(feature = "server", feature = "client"))]
mod grpc;
#[cfg(any(feature = "server", feature = "client"))]
pub(crate) use self::grpc::define_grpc;
