use std::collections::HashMap;

use prost::{Message, bytes::Bytes};
use prost_types::Struct;

/// The X509SVIDRequest message conveys parameters for requesting an X.509-SVID.
/// There are currently no request parameters.
#[derive(Clone, Copy, PartialEq, Message)]
pub struct X509SvidRequest {}

/// The X509SVIDResponse message carries X.509-SVIDs and related information,
/// including a set of global CRLs and a list of bundles the workload may use
/// for federating with foreign trust domains.
#[derive(Clone, PartialEq, Message)]
pub struct X509SvidResponse {
    /// Required. A list of X509SVID messages, each of which includes a single
    /// X.509-SVID, its private key, and the bundle for the trust domain.
    #[prost(message, repeated, tag = "1")]
    pub svids: Vec<X509Svid>,

    /// Optional. ASN.1 DER encoded certificate revocation lists.
    #[prost(bytes = "bytes", repeated, tag = "2")]
    pub crl: Vec<Bytes>,

    /// Optional. CA certificate bundles belonging to foreign trust domains that
    /// the workload should trust, keyed by the SPIFFE ID of the foreign trust
    /// domain. Bundles are ASN.1 DER encoded.
    #[prost(map = "string, bytes", tag = "3")]
    pub federated_bundles: HashMap<String, Bytes>,
}

/// The X509SVID message carries a single SVID and all associated information,
/// including the X.509 bundle for the trust domain.
#[derive(Clone, PartialEq, Message)]
pub struct X509Svid {
    /// Required. The SPIFFE ID of the SVID in this entry
    #[prost(string, tag = "1")]
    pub spiffe_id: String,

    /// Required. ASN.1 DER encoded certificate chain. MAY include
    /// intermediates, the leaf certificate (or SVID itself) MUST come first.
    #[prost(bytes = "bytes", tag = "2")]
    pub x509_svid: Bytes,

    /// Required. ASN.1 DER encoded PKCS#8 private key. MUST be unencrypted.
    #[prost(bytes = "bytes", tag = "3")]
    pub x509_svid_key: Bytes,

    /// Required. ASN.1 DER encoded X.509 bundle for the trust domain.
    #[prost(bytes = "bytes", tag = "4")]
    pub bundle: Bytes,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[prost(string, tag = "5")]
    pub hint: String,
}

/// The X509BundlesRequest message conveys parameters for requesting X.509
/// bundles. There are currently no such parameters.
#[derive(Clone, Copy, PartialEq, Message)]
pub struct X509BundlesRequest {}

/// The X509BundlesResponse message carries a set of global CRLs and a map of
/// trust bundles the workload should trust.
#[derive(Clone, PartialEq, Message)]
pub struct X509BundlesResponse {
    /// Optional. ASN.1 DER encoded certificate revocation lists.
    #[prost(bytes = "bytes", repeated, tag = "1")]
    pub crl: Vec<Bytes>,

    /// Required. CA certificate bundles belonging to trust domains that the
    /// workload should trust, keyed by the SPIFFE ID of the trust domain.
    /// Bundles are ASN.1 DER encoded.
    #[prost(map = "string, bytes", tag = "2")]
    pub bundles: HashMap<String, Bytes>,
}

#[derive(Clone, PartialEq, Message)]
pub struct JwtSvidRequest {
    /// Required. The audience(s) the workload intends to authenticate against.
    #[prost(string, repeated, tag = "1")]
    pub audience: Vec<String>,

    /// Optional. The requested SPIFFE ID for the JWT-SVID. If unset, all
    /// JWT-SVIDs to which the workload is entitled are requested.
    #[prost(string, tag = "2")]
    pub spiffe_id: String,
}

/// The JWTSVIDResponse message conveys JWT-SVIDs.
#[derive(Clone, PartialEq, Message)]
pub struct JwtSvidResponse {
    /// Required. The list of returned JWT-SVIDs.
    #[prost(message, repeated, tag = "1")]
    pub svids: Vec<JwtSvid>,
}

/// The JWTSVID message carries the JWT-SVID token and associated metadata.
#[derive(Clone, PartialEq, Message)]
pub struct JwtSvid {
    /// Required. The SPIFFE ID of the JWT-SVID.
    #[prost(string, tag = "1")]
    pub spiffe_id: String,

    /// Required. Encoded JWT using JWS Compact Serialization.
    #[prost(string, tag = "2")]
    pub svid: String,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[prost(string, tag = "3")]
    pub hint: String,
}

/// The JWTBundlesRequest message conveys parameters for requesting JWT bundles.
/// There are currently no such parameters.
#[derive(Clone, Copy, PartialEq, Message)]
pub struct JwtBundlesRequest {}

/// The JWTBundlesReponse conveys JWT bundles.
#[derive(Clone, PartialEq, Message)]
pub struct JwtBundlesResponse {
    /// Required. JWK encoded JWT bundles, keyed by the SPIFFE ID of the trust
    /// domain.
    #[prost(map = "string, bytes", tag = "1")]
    pub bundles: HashMap<String, Bytes>,
}

/// The ValidateJWTSVIDRequest message conveys request parameters for
/// JWT-SVID validation.
#[derive(Clone, PartialEq, Message)]
pub struct ValidateJwtSvidRequest {
    /// Required. The audience of the validating party. The JWT-SVID must
    /// contain an audience claim which contains this value in order to
    /// succesfully validate.
    #[prost(string, tag = "1")]
    pub audience: String,

    /// Required. The JWT-SVID to validate, encoded using JWS Compact
    /// Serialization.
    #[prost(string, tag = "2")]
    pub svid: String,
}

/// The ValidateJWTSVIDReponse message conveys the JWT-SVID validation results.
#[derive(Clone, PartialEq, Message)]
pub struct ValidateJwtSvidResponse {
    /// Required. The SPIFFE ID of the validated JWT-SVID.
    #[prost(string, tag = "1")]
    pub spiffe_id: String,
    /// Required. Claims contained within the payload of the validated JWT-SVID.
    /// This includes both SPIFFE-required and non-required claims.
    #[prost(message, optional, tag = "2")]
    pub claims: Option<Struct>,
}
