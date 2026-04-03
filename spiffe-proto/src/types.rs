use std::collections::HashMap;

use bytes::Buf;
use buffa::{__private::CachedSize, MessageField, UnknownFields, view::{MapView, RepeatedView, UnknownFieldsView}};
use buffa_derive::{BuffaMessage, BuffaMessageView};
use buffa_types::Struct;

/// The X509SVIDRequest message conveys parameters for requesting an X.509-SVID.
/// There are currently no request parameters.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "X509SVIDRequest")]
pub struct X509SvidRequest {
    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The X509SVIDResponse message carries X.509-SVIDs and related information,
/// including a set of global CRLs and a list of bundles the workload may use
/// for federating with foreign trust domains.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "X509SVIDResponse")]
pub struct X509SvidResponse {
    /// Required. A list of X509SVID messages, each of which includes a single
    /// X.509-SVID, its private key, and the bundle for the trust domain.
    #[buffa(message, repeated, tag = 1)]
    pub svids: Vec<X509Svid>,

    /// Optional. ASN.1 DER encoded certificate revocation lists.
    #[buffa(bytes, repeated, tag = 2)]
    pub crl: Vec<Vec<u8>>,

    /// Optional. CA certificate bundles belonging to foreign trust domains that
    /// the workload should trust, keyed by the SPIFFE ID of the foreign trust
    /// domain. Bundles are ASN.1 DER encoded.
    #[buffa(map, key = string, value = bytes, tag = 3)]
    pub federated_bundles: HashMap<String, Vec<u8>>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The X509SVID message carries a single SVID and all associated information,
/// including the X.509 bundle for the trust domain.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "X509SVID")]
pub struct X509Svid {
    /// Required. The SPIFFE ID of the SVID in this entry
    #[buffa(string, tag = 1)]
    pub spiffe_id: String,

    /// Required. ASN.1 DER encoded certificate chain. MAY include
    /// intermediates, the leaf certificate (or SVID itself) MUST come first.
    #[buffa(bytes, tag = 2)]
    pub x509_svid: Vec<u8>,

    /// Required. ASN.1 DER encoded PKCS#8 private key. MUST be unencrypted.
    #[buffa(bytes, tag = 3)]
    pub x509_svid_key: Vec<u8>,

    /// Required. ASN.1 DER encoded X.509 bundle for the trust domain.
    #[buffa(bytes, tag = 4)]
    pub bundle: Vec<u8>,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[buffa(string, tag = 5)]
    pub hint: String,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "X509Svid")]
pub struct X509SvidView<'a> {
    #[buffa(string, tag = 1)]
    pub spiffe_id: &'a str,

    #[buffa(bytes, tag = 2)]
    pub x509_svid: &'a [u8],

    #[buffa(bytes, tag = 3)]
    pub x509_svid_key: &'a [u8],

    #[buffa(bytes, tag = 4)]
    pub bundle: &'a [u8],

    #[buffa(string, tag = 5)]
    pub hint: &'a str,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "X509SvidResponse")]
pub struct X509SvidResponseView<'a> {
    #[buffa(message, repeated, tag = 1)]
    pub svids: RepeatedView<'a, X509SvidView<'a>>,

    #[buffa(bytes, repeated, tag = 2)]
    pub crl: RepeatedView<'a, &'a [u8]>,

    #[buffa(map, key = string, value = bytes, tag = 3)]
    pub federated_bundles: MapView<'a, &'a str, &'a [u8]>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

/// The X509BundlesRequest message conveys parameters for requesting X.509
/// bundles. There are currently no such parameters.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "X509BundlesRequest")]
pub struct X509BundlesRequest {
    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The X509BundlesResponse message carries a set of global CRLs and a map of
/// trust bundles the workload should trust.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "X509BundlesResponse")]
pub struct X509BundlesResponse {
    /// Optional. ASN.1 DER encoded certificate revocation lists.
    #[buffa(bytes, repeated, tag = 1)]
    pub crl: Vec<Vec<u8>>,

    /// Required. CA certificate bundles belonging to trust domains that the
    /// workload should trust, keyed by the SPIFFE ID of the trust domain.
    /// Bundles are ASN.1 DER encoded.
    #[buffa(map, key = string, value = bytes, tag = 2)]
    pub bundles: HashMap<String, Vec<u8>>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "X509BundlesResponse")]
pub struct X509BundlesResponseView<'a> {
    #[buffa(bytes, repeated, tag = 1)]
    pub crl: RepeatedView<'a, &'a [u8]>,

    #[buffa(map, key = string, value = bytes, tag = 2)]
    pub bundles: MapView<'a, &'a str, &'a [u8]>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "JWTSVIDRequest")]
pub struct JwtSvidRequest {
    /// Required. The audience(s) the workload intends to authenticate against.
    #[buffa(string, repeated, tag = 1)]
    pub audience: Vec<String>,

    /// Optional. The requested SPIFFE ID for the JWT-SVID. If unset, all
    /// JWT-SVIDs to which the workload is entitled are requested.
    #[buffa(string, tag = 2)]
    pub spiffe_id: String,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The JWTSVIDResponse message conveys JWT-SVIDs.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "JWTSVIDResponse")]
pub struct JwtSvidResponse {
    /// Required. The list of returned JWT-SVIDs.
    #[buffa(message, repeated, tag = 1)]
    pub svids: Vec<JwtSvid>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "JwtSvidRequest")]
pub struct JwtSvidRequestView<'a> {
    #[buffa(string, repeated, tag = 1)]
    pub audience: RepeatedView<'a, &'a str>,

    #[buffa(string, tag = 2)]
    pub spiffe_id: &'a str,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "JwtSvid")]
pub struct JwtSvidView<'a> {
    #[buffa(string, tag = 1)]
    pub spiffe_id: &'a str,

    #[buffa(string, tag = 2)]
    pub svid: &'a str,

    #[buffa(string, tag = 3)]
    pub hint: &'a str,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

#[derive(Default, Clone, BuffaMessageView)]
#[buffa(owned = "JwtSvidResponse")]
pub struct JwtSvidResponseView<'a> {
    #[buffa(message, repeated, tag = 1)]
    pub svids: RepeatedView<'a, JwtSvidView<'a>>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFieldsView<'a>,
}

/// The JWTSVID message carries the JWT-SVID token and associated metadata.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "JWTSVID")]
pub struct JwtSvid {
    /// Required. The SPIFFE ID of the JWT-SVID.
    #[buffa(string, tag = 1)]
    pub spiffe_id: String,

    /// Required. Encoded JWT using JWS Compact Serialization.
    #[buffa(string, tag = 2)]
    pub svid: String,

    /// Optional. An operator-specified string used to provide guidance on how this
    /// identity should be used by a workload when more than one SVID is returned.
    /// For example, `internal` and `external` to indicate an SVID for internal or
    /// external use, respectively.
    #[buffa(string, tag = 3)]
    pub hint: String,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The JWTBundlesRequest message conveys parameters for requesting JWT bundles.
/// There are currently no such parameters.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "JWTBundlesRequest")]
pub struct JwtBundlesRequest {
    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The JWTBundlesReponse conveys JWT bundles.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "JWTBundlesResponse")]
pub struct JwtBundlesResponse {
    /// Required. JWK encoded JWT bundles, keyed by the SPIFFE ID of the trust
    /// domain.
    #[buffa(map, key = string, value = bytes, tag = 1)]
    pub bundles: HashMap<String, Vec<u8>>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The ValidateJWTSVIDRequest message conveys request parameters for
/// JWT-SVID validation.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "ValidateJWTSVIDRequest")]
pub struct ValidateJwtSvidRequest {
    /// Required. The audience of the validating party. The JWT-SVID must
    /// contain an audience claim which contains this value in order to
    /// succesfully validate.
    #[buffa(string, tag = 1)]
    pub audience: String,

    /// Required. The JWT-SVID to validate, encoded using JWS Compact
    /// Serialization.
    #[buffa(string, tag = 2)]
    pub svid: String,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}

/// The ValidateJWTSVIDReponse message conveys the JWT-SVID validation results.
#[derive(Default, Clone, PartialEq, BuffaMessage)]
#[buffa(name = "ValidateJWTSVIDResponse")]
pub struct ValidateJwtSvidResponse {
    /// Required. The SPIFFE ID of the validated JWT-SVID.
    #[buffa(string, tag = 1)]
    pub spiffe_id: String,

    /// Required. Claims contained within the payload of the validated JWT-SVID.
    /// This includes both SPIFFE-required and non-required claims.
    #[buffa(message, tag = 2)]
    pub claims: MessageField<Struct>,

    #[doc(hidden)]
    pub __buffa_unknown_fields: UnknownFields,

    #[doc(hidden)]
    pub __buffa_cached_size: CachedSize,
}
