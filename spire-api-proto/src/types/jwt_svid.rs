use prost::Message;

use super::SpiffeId;

/// JWT SPIFFE Verifiable Identity Document. It contains the raw JWT token
/// as well as a few denormalized fields for convenience.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct JwtSvid {
    /// The serialized JWT token.
    #[prost(string, tag = "1")]
    pub token: String,

    /// The SPIFFE ID of the JWT-SVID.
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
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[prost(string, tag = "5")]
    pub hint: String,
}
