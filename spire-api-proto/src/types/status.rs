use prost::Message;

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct Status {
    /// A status code, which should be an enum value of google.rpc.Code.
    #[prost(int32, tag = "1")]
    pub code: i32,

    /// A developer-facing error message.
    #[prost(string, tag = "2")]
    pub message: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PermissionDeniedDetails {
    /// The reason for permission denied.
    #[prost(enumeration = "permission_denied_details::Reason", tag = "1")]
    pub reason: i32,
}

pub mod permission_denied_details {
    use prost::Enumeration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        /// Reason unknown.
        Unknown = 0,
        /// Agent identity has expired.
        AgentExpired = 1,
        /// Identity is not an attested agent.
        AgentNotAttested = 2,
        /// Identity is not the active agent identity.
        AgentNotActive = 3,
        /// Agent has been banned.
        AgentBanned = 4,
        /// Agent attempted to renew SVID, but should reattest instead.
        AgentMustReattest = 5,
    }

    impl Reason {
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unknown => "UNKNOWN",
                Self::AgentExpired => "AGENT_EXPIRED",
                Self::AgentNotAttested => "AGENT_NOT_ATTESTED",
                Self::AgentNotActive => "AGENT_NOT_ACTIVE",
                Self::AgentBanned => "AGENT_BANNED",
                Self::AgentMustReattest => "AGENT_MUST_REATTEST",
            }
        }

        pub fn from_str_name(value: &str) -> Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "AGENT_EXPIRED" => Some(Self::AgentExpired),
                "AGENT_NOT_ATTESTED" => Some(Self::AgentNotAttested),
                "AGENT_NOT_ACTIVE" => Some(Self::AgentNotActive),
                "AGENT_BANNED" => Some(Self::AgentBanned),
                "AGENT_MUST_REATTEST" => Some(Self::AgentMustReattest),
                _ => None,
            }
        }
    }
}
