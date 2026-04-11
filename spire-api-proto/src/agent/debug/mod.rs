mod types;

use tonic_service::define_grpc;

pub use self::types::*;

define_grpc! {
    Debug,
    DebugClient,
    DebugServer,
    "spire.agent.debug.v1.Debug",

    /// Get information about SPIRE agent
    fn get_info("GetInfo")(GetInfoRequest) -> (GetInfoResponse);
}
