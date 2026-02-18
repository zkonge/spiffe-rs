use prost::Message;

use super::SpiffeId;

/// WIT SPIFFE Verifiable Identity Document. It contains the raw WIT token
/// as well as a few denormalized fields for convenience.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct WitSvid {
    /// The serialized WIT token.
    #[prost(string, tag = "1")]
    pub token: String,

    /// The SPIFFE ID of the WIT-SVID.
    #[prost(message, optional, tag = "2")]
    pub id: Option<SpiffeId>,

    /// Expiration timestamp (seconds since Unix epoch).
    #[prost(int64, tag = "3")]
    pub expires_at: i64,

    /// Issuance timestamp (seconds since Unix epoch).
    #[prost(int64, tag = "4")]
    pub issued_at: i64,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    #[prost(string, tag = "5")]
    pub hint: String,
}
