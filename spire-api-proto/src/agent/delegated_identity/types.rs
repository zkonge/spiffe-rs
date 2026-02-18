use std::collections::HashMap;

use prost::{Message, bytes::Bytes};

use crate::{JwtSvid, Selector, X509Svid};

/// X.509 SPIFFE Verifiable Identity Document with the private key.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct X509SvidWithKey {
    /// The workload X509-SVID.
    #[prost(message, optional, tag = "1")]
    pub x509_svid: Option<X509Svid>,

    /// Private key (encoding DER PKCS#8).
    #[prost(bytes = "bytes", tag = "2")]
    pub x509_svid_key: Bytes,
}

/// SubscribeToX509SVIDsRequest is used by clients to subscribe the set of SVIDs that
/// any given workload is entitled to. Clients subscribe to a workload's SVIDs by providing
/// one-of
///
/// * a set of selectors describing the workload.
/// * a PID of a workload process.
///   Specifying both at the same time is not allowed.
///
/// Subscribers are expected to ensure that the PID they use is not recycled
/// for the lifetime of the stream, and in the event that it is, are expected
/// to immediately close the stream.
///
/// TODO we should use `oneof` here but you currently cannot use `repeated`
/// in a `oneof` without creating and nesting an intermediate `message` type, which would break
/// back compat - so we accept both and check for mutual exclusion in the handler
#[derive(Clone, PartialEq, Message)]
pub struct SubscribeToX509SvidsRequest {
    /// Selectors describing the workload to subscribe to. Mutually exclusive with `pid`.
    #[prost(message, repeated, tag = "1")]
    pub selectors: Vec<Selector>,

    /// PID for the workload to subscribe to. Mutually exclusive with `selectors`
    #[prost(int32, tag = "2")]
    pub pid: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct SubscribeToX509SvidsResponse {
    #[prost(message, repeated, tag = "1")]
    pub x509_svids: Vec<X509SvidWithKey>,
    /// Names of the trust domains that this workload should federates with.
    #[prost(string, repeated, tag = "2")]
    pub federates_with: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct SubscribeToX509BundlesRequest {}

/// SubscribeToX509BundlesResponse contains all bundles that the agent is tracking,
/// including the local bundle. When an update occurs, or bundles are added or removed,
/// a new response with the full set of bundles is sent.
#[derive(Clone, PartialEq, Message)]
pub struct SubscribeToX509BundlesResponse {
    /// A map keyed by trust domain name, with ASN.1 DER-encoded
    /// X.509 CA certificates as the values
    #[prost(map = "string, bytes", tag = "1")]
    pub ca_certificates: HashMap<String, Bytes>,
}

/// FetchJWTSVIDsRequest is used by clients to fetch a JWT-SVID for a workload.
/// Clients may provide one-of
///
/// * a set of selectors describing the workload.
/// * a PID of a workload process.
///   Specifying both at the same time is not allowed.
///
/// Callers are expected to ensure that the PID they use is not recycled
/// until obtaining a response, and in the event that it is, are expected
/// to discard the response of this call.
///
/// TODO we should use `oneof` here but you currently cannot use `repeated`
/// in a `oneof` without creating and nesting an intermediate `message` type, which would break
/// back compat - so we accept both and check for mutual exclusion in the handler
#[derive(Clone, PartialEq, Message)]
pub struct FetchJwtSvidsRequest {
    /// Required. The audience(s) the workload intends to authenticate against.
    #[prost(string, repeated, tag = "1")]
    pub audience: Vec<String>,

    /// Selectors describing the workload to subscribe to. Mutually exclusive with `pid`
    #[prost(message, repeated, tag = "2")]
    pub selectors: Vec<Selector>,

    /// PID for the workload to subscribe to. Mutually exclusive with `selectors`.
    #[prost(int32, tag = "3")]
    pub pid: i32,
}

/// The FetchJWTSVIDsResponse message conveys JWT-SVIDs.
#[derive(Clone, PartialEq, Message)]
pub struct FetchJwtSvidsResponse {
    /// Required. The list of returned JWT-SVIDs.
    #[prost(message, repeated, tag = "1")]
    pub svids: Vec<JwtSvid>,
}

/// The SubscribeToJWTBundlesRequest message conveys parameters for requesting JWKS bundles.
/// There are currently no such parameters.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct SubscribeToJwtBundlesRequest {}

/// The SubscribeToJWTBundlesResponse conveys JWKS bundles.
#[derive(Clone, PartialEq, Message)]
pub struct SubscribeToJwtBundlesResponse {
    /// Required. JWK encoded JWT bundles, keyed by the SPIFFE ID of the trust
    /// domain.
    #[prost(map = "string, bytes", tag = "1")]
    pub bundles: HashMap<String, Bytes>,
}
