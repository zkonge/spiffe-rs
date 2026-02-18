use prost::{Enumeration, Message};

/// Represents the current Logger settings.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct Logger {
    /// Output only. The logger's current log level.
    #[prost(enumeration = "LogLevel", tag = "1")]
    pub current_level: i32,

    /// Output only. The logger's log level at process launch.
    #[prost(enumeration = "LogLevel", tag = "2")]
    pub launch_level: i32,
}

/// The logger log levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    Unspecified = 0,
    Panic = 1,
    Fatal = 2,
    Error = 3,
    Warn = 4,
    Info = 5,
    Debug = 6,
    Trace = 7,
}

impl LogLevel {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Panic => "PANIC",
            Self::Fatal => "FATAL",
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "PANIC" => Some(Self::Panic),
            "FATAL" => Some(Self::Fatal),
            "ERROR" => Some(Self::Error),
            "WARN" => Some(Self::Warn),
            "INFO" => Some(Self::Info),
            "DEBUG" => Some(Self::Debug),
            "TRACE" => Some(Self::Trace),
            _ => None,
        }
    }
}
