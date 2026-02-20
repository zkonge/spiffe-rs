use prost::Message;

use crate::{JwtSvid, SpiffeId, WitSvid, X509Svid};

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct MintX509SvidRequest {
    /// Required. ASN.1 DER encoded CSR. The CSR is used to convey the public
    /// key and the SPIFFE ID (via the URI SAN). Only one URI SAN can be set.
    /// Optionally, the subject and any number of DNS SANs can also be set.
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,

    /// The desired TTL of the X509-SVID, in seconds. The server default will be
    /// used if unset. The TTL is advisory only. The actual lifetime of the
    /// X509-SVID may be lower depending on the remaining lifetime of the active
    /// SPIRE Server CA.
    #[prost(int32, tag = "2")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintX509SvidResponse {
    /// The newly issued X509-SVID.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<X509Svid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintWitSvidRequest {
    /// Required. SPIFFE ID of the WIT-SVID.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    /// Required. The ASN.1 DER encoded public key.
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: Vec<u8>,

    /// Desired TTL of the WIT-SVID, in seconds. The server default will be used
    /// if unset. The TTL is advisory only. The actual lifetime of the WIT-SVID
    /// may be lower depending on the remaining lifetime of the active SPIRE
    /// Server CA.
    #[prost(int32, tag = "3")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintWitSvidResponse {
    /// The newly issued WIT-SVID.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<WitSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintJwtSvidRequest {
    /// Required. SPIFFE ID of the JWT-SVID.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    /// Required. List of audience claims to include in the JWT-SVID. At least one must
    /// be set.
    #[prost(string, repeated, tag = "2")]
    pub audience: Vec<String>,

    /// Desired TTL of the JWT-SVID, in seconds. The server default will be used
    /// if unset. The TTL is advisory only. The actual lifetime of the JWT-SVID
    /// may be lower depending on the remaining lifetime of the active SPIRE
    /// Server CA.
    #[prost(int32, tag = "3")]
    pub ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct MintJwtSvidResponse {
    /// The newly issued JWT-SVID.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<JwtSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewX509SvidRequest {
    /// Required. One or more X509-SVID parameters for X509-SVID entries to
    /// be signed.
    #[prost(message, repeated, tag = "1")]
    pub params: Vec<NewX509SvidParams>,
}

pub mod batch_new_x509_svid_response {
    use prost::Message;

    use crate::{Status, X509Svid};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the X509-SVID.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The newly created X509-SVID. This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub svid: Option<X509Svid>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewX509SvidResponse {
    /// Result for each X509-SVID requested (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_new_x509_svid_response::Result>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewJwtSvidRequest {
    /// Required. The entry ID of the identity being requested.
    #[prost(string, tag = "1")]
    pub entry_id: String,

    /// Required. List of audience claims to include in the JWT-SVID. At least
    #[prost(string, repeated, tag = "2")]
    pub audience: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct NewJwtSvidResponse {
    /// The newly issued JWT-SVID.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<JwtSvid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewWitSvidRequest {
    /// Required. One or more WIT-SVID parameters for WIT-SVID entries to
    /// be signed.
    #[prost(message, repeated, tag = "1")]
    pub params: Vec<NewWitSvidParams>,
}

pub mod batch_new_wit_svid_response {
    use prost::Message;

    use crate::{Status, WitSvid};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the WIT-SVID.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The newly created WIT-SVID. This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub svid: Option<WitSvid>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchNewWitSvidResponse {
    /// Result for each WIT-SVID requested (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_new_wit_svid_response::Result>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewDownstreamX509CaRequest {
    /// Required. The ASN.1 DER encoded Certificate Signing Request (CSR). The
    /// CSR is only used to convey the public key; other fields in the CSR are
    /// ignored. The X509-SVID attributes are determined by the downstream entry.
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,

    /// Optional. The TTL preferred by the downstream SPIRE Server for the
    /// signed intermediate CA. If zero, the upstream SPIRE Server will use its
    /// own default.
    #[prost(int32, tag = "2")]
    pub preferred_ttl: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct NewDownstreamX509CaResponse {
    /// CA certificate and any intermediates required to form a chain of trust
    /// back to the X.509 authorities (DER encoded). The CA certificate is the
    /// first.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub ca_cert_chain: Vec<Vec<u8>>,

    /// X.509 authorities (DER encoded).
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub x509_authorities: Vec<Vec<u8>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewX509SvidParams {
    /// Required. The entry ID for the identity being requested.
    #[prost(string, tag = "1")]
    pub entry_id: String,

    /// Required. The ASN.1 DER encoded Certificate Signing Request (CSR). The
    /// CSR is only used to convey the public key; other fields in the CSR are
    /// ignored. The X509-SVID attributes are determined by the entry.
    #[prost(bytes = "vec", tag = "2")]
    pub csr: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct NewWitSvidParams {
    /// Required. The entry ID for the identity being requested.
    #[prost(string, tag = "1")]
    pub entry_id: String,

    /// Required. The ASN.1 DER encoded public key.
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: Vec<u8>,
}
