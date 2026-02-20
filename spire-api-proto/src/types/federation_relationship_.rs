use prost::Message;

use super::Bundle;

pub mod federation_relationship {
    use prost::Oneof;

    use super::{HttpsSpiffeProfile, HttpsWebProfile};

    /// Required. The endpoint profile type.
    #[derive(Clone, PartialEq, Eq, Hash, Oneof)]
    pub enum BundleEndpointProfile {
        /// Use Web PKI endpoint profile.
        #[prost(message, tag = "3")]
        HttpsWeb(HttpsWebProfile),

        /// Use SPIFFE Authentication endpoint profile.
        #[prost(message, tag = "4")]
        HttpsSpiffe(HttpsSpiffeProfile),
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct FederationRelationship {
    /// Required. The trust domain name (e.g., "example.org") to federate with.
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// Required. URL of the SPIFFE bundle endpoint that provides the trust
    /// bundle to federate with. Must use the HTTPS protocol.
    #[prost(string, tag = "2")]
    pub bundle_endpoint_url: String,

    /// Required. The endpoint profile type.
    #[prost(
        oneof = "federation_relationship::BundleEndpointProfile",
        tags = "3, 4"
    )]
    pub bundle_endpoint_profile: Option<federation_relationship::BundleEndpointProfile>,

    /// Optional. The bundle for the trust domain. This field can be used to
    /// create or replace the referenced trust domains' bundle when the
    /// relationship is created or updated.  When the relationship is retrieved,
    /// it will be set to the referenced trust domain's latest bundle (if
    /// available). Please note that the `https_spiffe` profile requires an
    /// existing trust domain bundle in order to function correctly. The
    /// required bundle must match the trust domain specified in the bundle
    /// endpoint SPIFFE ID. If the bundle endpoint SPIFFE ID resides in the same
    /// trust domain that you're trying to federate with, you may optionally
    /// specify that trust domain bundle here. If the bundle endpoint SPIFFE ID
    /// _does not_ reside in the same trust domain that you're federating with,
    /// please ensure that the trust domain bundle for that trust domain has
    /// been configured separately (e.g. configured via another federation
    /// relationship or manually set via the Bundle API).
    #[prost(message, optional, tag = "5")]
    pub trust_domain_bundle: Option<Bundle>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct HttpsSpiffeProfile {
    /// Required. Specifies the expected SPIFFE ID of the SPIFFE bundle endpoint server.
    #[prost(string, tag = "1")]
    pub endpoint_spiffe_id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct HttpsWebProfile {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct FederationRelationshipMask {
    /// bundle_endpoint_url field mask.
    #[prost(bool, tag = "1")]
    pub bundle_endpoint_url: bool,

    /// bundle_endpoint_profile field mask.
    #[prost(bool, tag = "2")]
    pub bundle_endpoint_profile: bool,

    /// trust_domain_bundle field mask.
    #[prost(bool, tag = "3")]
    pub trust_domain_bundle: bool,
}
