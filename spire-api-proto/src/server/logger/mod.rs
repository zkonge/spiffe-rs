mod types;

pub use self::types::*;
use crate::{Logger, macros::define_grpc};

define_grpc! {
    /// Logger service for SPIRE Server.
    LoggerService,
    LoggerServiceClient,
    LoggerServiceServer,
    "spire.api.server.logger.v1.Logger",

    fn get_logger("GetLogger")(GetLoggerRequest) -> (Logger);

    fn set_log_level("SetLogLevel")(SetLogLevelRequest) -> (Logger);

    fn reset_log_level("ResetLogLevel")(ResetLogLevelRequest) -> (Logger);
}
