mod types;

pub use self::types::*;
use crate::{FederationRelationship, macros::define_grpc};

define_grpc! {
    /// Manages the federation relationships with foreign trust domains.
    TrustDomain,
    TrustDomainClient,
    TrustDomainServer,
    "spire.api.server.trustdomain.v1.TrustDomain",

    /// Lists federation relationships with foreign trust domains.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn list_federation_relationships("ListFederationRelationships")(ListFederationRelationshipsRequest) -> (ListFederationRelationshipsResponse);

    /// Gets a federation relationship with a foreign trust domain.
    /// If there is no federation relationship with the specified
    /// trust domain, NOT_FOUND is returned.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn get_federation_relationship("GetFederationRelationship")(GetFederationRelationshipRequest) -> (FederationRelationship);

    /// Batch creates one or more federation relationships with
    /// foreign trust domains.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn batch_create_federation_relationship("BatchCreateFederationRelationship")(BatchCreateFederationRelationshipRequest) -> (BatchCreateFederationRelationshipResponse);

    /// Batch updates one or more federation relationships with
    /// foreign trust domains.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn batch_update_federation_relationship("BatchUpdateFederationRelationship")(BatchUpdateFederationRelationshipRequest) -> (BatchUpdateFederationRelationshipResponse);

    /// Batch deletes federation relationships with foreign trust domains.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn batch_delete_federation_relationship("BatchDeleteFederationRelationship")(BatchDeleteFederationRelationshipRequest) -> (BatchDeleteFederationRelationshipResponse);

    /// Refreshes the bundle from the specified federated trust domain.
    /// If there is not a federation relationship configured with the
    /// specified trust domain, NOT_FOUND is returned.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn refresh_bundle("RefreshBundle")(RefreshBundleRequest) -> (());
}
