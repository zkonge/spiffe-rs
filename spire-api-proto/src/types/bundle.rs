use prost::{Message, bytes::Bytes};

#[derive(Clone, PartialEq, Message)]
pub struct Bundle {
    /// The name of the trust domain the bundle belongs to (e.g., "example.org").
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// X.509 authorities for authenticating X509-SVIDs.
    #[prost(message, repeated, tag = "2")]
    pub x509_authorities: Vec<X509Certificate>,

    /// JWT authorities for authenticating JWT-SVIDs.
    #[prost(message, repeated, tag = "3")]
    pub jwt_authorities: Vec<JwtKey>,

    /// A hint on how often the bundle should be refreshed from the bundle provider, in seconds.
    #[prost(int64, tag = "4")]
    pub refresh_hint: i64,

    /// The sequence number of the bundle.
    #[prost(uint64, tag = "5")]
    pub sequence_number: u64,

    /// WIT authorities for authenticating WIT-SVIDs.
    #[prost(message, repeated, tag = "6")]
    pub wit_authorities: Vec<WitKey>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct X509Certificate {
    /// The ASN.1 DER encoded bytes of the X.509 certificate.
    #[prost(bytes = "bytes", tag = "1")]
    pub asn1: Bytes,

    /// This authority is no longer secure and must not be used.
    #[prost(bool, tag = "2")]
    pub tainted: bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct JwtKey {
    /// The PKIX encoded public key.
    #[prost(bytes = "bytes", tag = "1")]
    pub public_key: Bytes,

    /// The key identifier.
    #[prost(string, tag = "2")]
    pub key_id: String,

    /// When the key expires (seconds since Unix epoch). If zero, the key does not expire.
    #[prost(int64, tag = "3")]
    pub expires_at: i64,

    /// This authority is no longer secure and must not be used.
    #[prost(bool, tag = "4")]
    pub tainted: bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct WitKey {
    /// The PKIX encoded public key.
    #[prost(bytes = "bytes", tag = "1")]
    pub public_key: Bytes,

    /// The key identifier.
    #[prost(string, tag = "2")]
    pub key_id: String,

    /// When the key expires (seconds since Unix epoch). If zero, the key does not expire.
    #[prost(int64, tag = "3")]
    pub expires_at: i64,

    /// This authority is no longer secure and must not be used.
    #[prost(bool, tag = "4")]
    pub tainted: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct BundleMask {
    /// x509_authorities field mask.
    #[prost(bool, tag = "2")]
    pub x509_authorities: bool,

    /// jwt_authorities field mask.
    #[prost(bool, tag = "3")]
    pub jwt_authorities: bool,

    /// refresh_hint field mask.
    #[prost(bool, tag = "4")]
    pub refresh_hint: bool,

    /// sequence_number field mask.
    #[prost(bool, tag = "5")]
    pub sequence_number: bool,

    /// wit_authorities field mask.
    #[prost(bool, tag = "6")]
    pub wit_authorities: bool,
}
