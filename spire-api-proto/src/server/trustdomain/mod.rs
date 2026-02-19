mod types;

pub use self::types::*;
use crate::{Empty, FederationRelationship, macros::define_grpc};

define_grpc! {
    /// Manages federation relationships with foreign trust domains.
    TrustDomain,
    TrustDomainClient,
    TrustDomainServer,
    "spire.api.server.trustdomain.v1.TrustDomain",

    fn list_federation_relationships("ListFederationRelationships")(ListFederationRelationshipsRequest) -> (ListFederationRelationshipsResponse);

    fn get_federation_relationship("GetFederationRelationship")(GetFederationRelationshipRequest) -> (FederationRelationship);

    fn batch_create_federation_relationship("BatchCreateFederationRelationship")(BatchCreateFederationRelationshipRequest) -> (BatchCreateFederationRelationshipResponse);

    fn batch_update_federation_relationship("BatchUpdateFederationRelationship")(BatchUpdateFederationRelationshipRequest) -> (BatchUpdateFederationRelationshipResponse);

    fn batch_delete_federation_relationship("BatchDeleteFederationRelationship")(BatchDeleteFederationRelationshipRequest) -> (BatchDeleteFederationRelationshipResponse);

    fn refresh_bundle("RefreshBundle")(RefreshBundleRequest) -> (Empty);
}
