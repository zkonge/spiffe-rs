mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    Debug,
    DebugClient,
    DebugServer,
    "spire.api.server.debug.v1.Debug",

    /// Get information about SPIRE server
    fn get_info("GetInfo")(GetInfoRequest) -> (GetInfoResponse);
}
