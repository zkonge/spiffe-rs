use prost::Message;

use crate::{FederationRelationship, FederationRelationshipMask, Status};

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct ListFederationRelationshipsRequest {
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<FederationRelationshipMask>,

    #[prost(int32, tag = "2")]
    pub page_size: i32,

    #[prost(string, tag = "3")]
    pub page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederationRelationshipsResponse {
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetFederationRelationshipRequest {
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<FederationRelationshipMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederationRelationshipRequest {
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<FederationRelationshipMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederationRelationshipResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchCreateFederationRelationshipResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederationRelationshipResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub federation_relationship: Option<FederationRelationship>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederationRelationshipRequest {
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<FederationRelationshipMask>,

    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<FederationRelationshipMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederationRelationshipResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchUpdateFederationRelationshipResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederationRelationshipResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(message, optional, tag = "2")]
    pub federation_relationship: Option<FederationRelationship>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct BatchDeleteFederationRelationshipRequest {
    #[prost(string, repeated, tag = "1")]
    pub trust_domains: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederationRelationshipResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchDeleteFederationRelationshipResponseResult>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederationRelationshipResponseResult {
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    #[prost(string, tag = "2")]
    pub trust_domain: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RefreshBundleRequest {
    #[prost(string, tag = "1")]
    pub trust_domain: String,
}
