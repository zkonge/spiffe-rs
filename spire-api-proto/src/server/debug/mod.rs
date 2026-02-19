mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// Debug service for SPIRE Server.
    Debug,
    DebugClient,
    DebugServer,
    "spire.api.server.debug.v1.Debug",

    fn get_info("GetInfo")(GetInfoRequest) -> (GetInfoResponse);
}
