use prost::{Message, bytes::Bytes};

use super::SpiffeId;

/// X.509 SPIFFE Verifiable Identity Document. It contains the raw X.509
/// certificate data as well as a few denormalized fields for convenience.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct X509Svid {
    /// Certificate and intermediates required to form a chain of trust back to
    /// the X.509 authorities of the trust domain (ASN.1 DER encoded).
    #[prost(bytes = "bytes", repeated, tag = "1")]
    pub cert_chain: Vec<Bytes>,

    /// SPIFFE ID of the SVID.
    #[prost(message, optional, tag = "2")]
    pub id: Option<SpiffeId>,

    /// Expiration timestamp (seconds since Unix epoch).
    #[prost(int64, tag = "3")]
    pub expires_at: i64,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[prost(string, tag = "4")]
    pub hint: String,
}
