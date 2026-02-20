use prost::Message;

use crate::{FederationRelationship, FederationRelationshipMask};

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct ListFederationRelationshipsRequest {
    /// An output mask indicating which federation replationship fields
    /// are set in the response.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<FederationRelationshipMask>,

    /// The maximum number of results to return. The server may further
    /// constrain this value, or if zero, choose its own.
    #[prost(int32, tag = "2")]
    pub page_size: i32,

    /// The next_page_token value returned from a previous request, if any.
    #[prost(string, tag = "3")]
    pub page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederationRelationshipsResponse {
    /// The federation relationships with foreign trust domains.
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    /// The page token for the next request. Empty if there are no more results.
    /// This field should be checked by clients even when a page_size was not
    /// requested, since the server may choose its own (see page_size).
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetFederationRelationshipRequest {
    /// Required. The trust domain name of the federation relationship
    /// (e.g., "example.org").
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// An output mask indicating which federation relationship fields
    /// are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<FederationRelationshipMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederationRelationshipRequest {
    /// The federation relationships to be created.
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    /// An output mask indicating the federation relationship fields set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<FederationRelationshipMask>,
}

mod batch_create_federation_relationship_response {
    use prost::Message;

    use crate::{FederationRelationship, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the federation relationship.
        /// Status code will be ALREADY_EXISTS if there is already a
        /// federation relationship with the specified trust domain.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The federation relationship that was created.
        /// This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub federation_relationship: Option<FederationRelationship>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederationRelationshipResponse {
    /// Result for each federation relationship in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_create_federation_relationship_response::Result>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederationRelationshipRequest {
    /// The federation relationships to be updated.
    #[prost(message, repeated, tag = "1")]
    pub federation_relationships: Vec<FederationRelationship>,

    /// An input mask indicating what federation relationship fields should be updated.
    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<FederationRelationshipMask>,

    /// An output mask indicating what federation relationship fields are set in the response.
    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<FederationRelationshipMask>,
}

mod batch_update_federation_relationship_response {
    use prost::Message;

    use crate::{FederationRelationship, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of updating the federation relationship.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The federation relationship that was updated.
        /// This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub federation_relationship: Option<FederationRelationship>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederationRelationshipResponse {
    /// Result for each federation relationship in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_update_federation_relationship_response::Result>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct BatchDeleteFederationRelationshipRequest {
    /// Required. The trust domain names of the federation relationships
    /// to delete.
    #[prost(string, repeated, tag = "1")]
    pub trust_domains: Vec<String>,
}

mod batch_delete_federation_relationship_response {
    use prost::Message;

    use crate::Status;
    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of delating the federation relationship.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The trust domain name of the federation relationship
        /// that was deleted.
        #[prost(string, tag = "2")]
        pub trust_domain: String,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederationRelationshipResponse {
    /// Result for each trust domain name in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_delete_federation_relationship_response::Result>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RefreshBundleRequest {
    /// Required. The federated trust domain name of the
    /// bundle to refresh (e.g., "example.org").
    #[prost(string, tag = "1")]
    pub trust_domain: String,
}
