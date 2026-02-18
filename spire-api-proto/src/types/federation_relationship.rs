use prost::Message;

use super::Bundle;

#[derive(Clone, PartialEq, Message)]
pub struct FederationRelationship {
    /// Required. The trust domain name (e.g., "example.org") to federate with.
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// Required. URL of the SPIFFE bundle endpoint that provides the trust bundle.
    #[prost(string, tag = "2")]
    pub bundle_endpoint_url: String,

    /// Optional. The bundle for the trust domain.
    #[prost(message, optional, tag = "5")]
    pub trust_domain_bundle: Option<Bundle>,

    /// Required. The endpoint profile type.
    #[prost(
        oneof = "bundle_endpoint_profile::BundleEndpointProfile",
        tags = "3, 4"
    )]
    pub bundle_endpoint_profile: Option<bundle_endpoint_profile::BundleEndpointProfile>,
}

pub mod bundle_endpoint_profile {
    use super::{HttpsSpiffeProfile, HttpsWebProfile};

    /// Required. The endpoint profile type.
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
    pub enum BundleEndpointProfile {
        /// Use Web PKI endpoint profile.
        #[prost(message, tag = "3")]
        HttpsWeb(HttpsWebProfile),

        /// Use SPIFFE Authentication endpoint profile.
        #[prost(message, tag = "4")]
        HttpsSpiffe(HttpsSpiffeProfile),
    }
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
