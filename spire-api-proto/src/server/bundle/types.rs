use prost::Message;

use crate::{Bundle as BundleType, BundleMask, JwtKey, Status, WitKey, X509Certificate};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct CountBundlesRequest {}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountBundlesResponse {
    #[prost(int32, tag = "1")]
    pub count: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct GetBundleRequest {
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AppendBundleRequest {
    #[prost(message, repeated, tag = "1")]
    pub x509_authorities: Vec<X509Certificate>,

    #[prost(message, repeated, tag = "2")]
    pub jwt_authorities: Vec<JwtKey>,

    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<BundleMask>,

    #[prost(message, repeated, tag = "4")]
    pub wit_authorities: Vec<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishJwtAuthorityRequest {
    #[prost(message, optional, tag = "1")]
    pub jwt_authority: Option<JwtKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishJwtAuthorityResponse {
    #[prost(message, repeated, tag = "1")]
    pub jwt_authorities: Vec<JwtKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishWitAuthorityRequest {
    #[prost(message, optional, tag = "1")]
    pub wit_authority: Option<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishWitAuthorityResponse {
    #[prost(message, repeated, tag = "1")]
    pub wit_authorities: Vec<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederatedBundlesRequest {
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<BundleMask>,

    #[prost(int32, tag = "2")]
    pub page_size: i32,

    #[prost(string, tag = "3")]
    pub page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederatedBundlesResponse {
    #[prost(message, repeated, tag = "1")]
    pub bundles: Vec<BundleType>,

    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetFederatedBundleRequest {
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederatedBundleRequest {
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederatedBundleResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchCreateFederatedBundleResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederatedBundleResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub bundle: Option<BundleType>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederatedBundleRequest {
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<BundleMask>,

    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederatedBundleResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchUpdateFederatedBundleResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederatedBundleResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub bundle: Option<BundleType>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchSetFederatedBundleRequest {
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchSetFederatedBundleResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchSetFederatedBundleResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchSetFederatedBundleResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub bundle: Option<BundleType>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederatedBundleRequest {
    #[prost(string, repeated, tag = "1")]
    pub trust_domains: Vec<String>,

    #[prost(enumeration = "batch_delete_federated_bundle_request::Mode", tag = "2")]
    pub mode: i32,
}

pub mod batch_delete_federated_bundle_request {
    use prost::Enumeration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        Restrict = 0,
        Delete = 1,
        Dissociate = 2,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederatedBundleResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchDeleteFederatedBundleResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederatedBundleResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(string, tag = "2")]
    pub trust_domain: String,
}
