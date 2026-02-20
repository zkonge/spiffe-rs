mod types;

pub use self::types::*;
use crate::{Logger as LoggerType, macros::define_grpc};

define_grpc! {
    Logger,
    LoggerClient,
    LoggerServer,
    "spire.api.server.logger.v1.Logger",

    /// Gets the logger level.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn get_logger("GetLogger")(GetLoggerRequest) -> (LoggerType);

    /// Sets the logger to a specified log level.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn set_log_level("SetLogLevel")(SetLogLevelRequest) -> (LoggerType);

    /// Resets the logger level to the level configured at launch.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn reset_log_level("ResetLogLevel")(ResetLogLevelRequest) -> (LoggerType);
}
