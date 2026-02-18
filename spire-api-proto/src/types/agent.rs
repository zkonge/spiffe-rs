use prost::Message;

use super::{Selector, SpiffeId};

#[derive(Clone, PartialEq, Message)]
pub struct Agent {
    /// Output only. SPIFFE ID of the agent.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    /// Output only. The method by which the agent attested.
    #[prost(string, tag = "2")]
    pub attestation_type: String,

    /// Output only. The X509-SVID serial number.
    #[prost(string, tag = "3")]
    pub x509_svid_serial_number: String,

    /// Output only. The X509-SVID expiration (seconds since Unix epoch).
    #[prost(int64, tag = "4")]
    pub x509_svid_expires_at: i64,

    /// Output only. The selectors attributed to the agent during attestation.
    #[prost(message, repeated, tag = "5")]
    pub selectors: Vec<Selector>,

    /// Output only. Whether or not the agent is banned.
    #[prost(bool, tag = "6")]
    pub banned: bool,

    /// Output only. Whether or not the agent can re-attest.
    #[prost(bool, tag = "7")]
    pub can_reattest: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct AgentMask {
    /// attestation_type field mask
    #[prost(bool, tag = "2")]
    pub attestation_type: bool,

    /// x509svid_serial_number field mask
    #[prost(bool, tag = "3")]
    pub x509_svid_serial_number: bool,

    /// x509svid_expires_at field mask
    #[prost(bool, tag = "4")]
    pub x509_svid_expires_at: bool,

    /// selectors field mask
    #[prost(bool, tag = "5")]
    pub selectors: bool,

    /// banned field mask
    #[prost(bool, tag = "6")]
    pub banned: bool,

    /// can_reattest field mask
    #[prost(bool, tag = "7")]
    pub can_reattest: bool,
}
