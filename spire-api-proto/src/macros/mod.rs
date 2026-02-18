mod client;
mod grpc;
mod server;

pub(crate) use self::{client::define_client, grpc::define_grpc, server::define_server};
