mod types;

pub use self::types::*;
use crate::{Logger as LoggerType, macros::define_grpc};

define_grpc! {
    Logger,
    LoggerClient,
    LoggerServer,
    "spire.api.agent.logger.v1.Logger",

    /// Gets the logger level.
    ///
    /// This message is intended for the Agent Admin Socket.
    fn get_logger("GetLogger")(GetLoggerRequest) -> (LoggerType);

    /// Sets the logger to a specified log level.
    ///
    /// This message is intended for the Agent Admin Socket.
    fn set_log_level("SetLogLevel")(SetLogLevelRequest) -> (LoggerType);

    /// Resets the logger level to the level configured at launch.
    ///
    /// This message is intended for the Agent Admin Socket.
    fn reset_log_level("ResetLogLevel")(ResetLogLevelRequest) -> (LoggerType);
}
