use prost::Message;

use crate::LogLevel;

// Empty Get Logger Request message for future extension
#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetLoggerRequest {}

// Set Log Level Request message
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct SetLogLevelRequest {
    /// The new level the logger should assume
    #[prost(enumeration = "LogLevel", tag = "1")]
    pub new_level: i32,
}

// Empty Reset Log Level Request message for future extension
#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct ResetLogLevelRequest {}
