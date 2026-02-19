use prost::Message;

use crate::LogLevel;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetLoggerRequest {}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct SetLogLevelRequest {
    #[prost(enumeration = "LogLevel", tag = "1")]
    pub new_level: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct ResetLogLevelRequest {}
