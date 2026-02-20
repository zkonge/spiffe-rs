mod types;

pub use self::types::*;
use crate::{Bundle as BundleType, macros::define_grpc};

define_grpc! {
    Bundle,
    BundleClient,
    BundleServer,
    "spire.api.server.bundle.v1.Bundle",

    /// Count bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn count_bundles("CountBundles")(CountBundlesRequest) -> (CountBundlesResponse);

    /// Gets the bundle for the trust domain of the server.
    //
    /// The RPC does not require authentication.
    fn get_bundle("GetBundle")(GetBundleRequest) -> (BundleType);

    /// Append to the bundle. Items specified in the bundle in the request are
    /// appended to the existing bundle. If the bundle does not exist, NOT_FOUND
    /// is returned. This is the only RPC that can be used to update the
    /// bundle for the trust domain of the SPIRE server.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn append_bundle("AppendBundle")(AppendBundleRequest) -> (BundleType);

    /// Publishes a downstream JWT authority to the SPIRE server. If the server
    /// is itself a downstream server (i.e. configured with an UpstreamAuthority
    /// plugin), the JWT authority is published further upstream using the
    /// UpstreamAuthority plugin. If the server is not a downstream server, or
    /// if the UpstreamAuthority does not support publishing JWT authorities,
    /// the server appends the JWT authority to its own bundle.
    //
    /// The caller must present a downstream X509-SVID.
    fn publish_jwt_authority("PublishJWTAuthority")(PublishJwtAuthorityRequest) -> (PublishJwtAuthorityResponse);

    /// Publishes a downstream WIT authority to the SPIRE server. If the server
    /// is itself a downstream server (i.e. configured with an UpstreamAuthority
    /// plugin), the WIT authority is published further upstream using the
    /// UpstreamAuthority plugin. If the server is not a downstream server, or
    /// if the UpstreamAuthority does not support publishing WIT authorities,
    /// the server appends the WIT authority to its own bundle.
    //
    /// The caller must present a downstream X509-SVID.
    fn publish_wit_authority("PublishWITAuthority")(PublishWitAuthorityRequest) -> (PublishWitAuthorityResponse);

    /// Lists federated bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn list_federated_bundles("ListFederatedBundles")(ListFederatedBundlesRequest) -> (ListFederatedBundlesResponse);

    /// Gets a federated bundle. If the bundle does not exist, NOT_FOUND is returned.
    //
    /// The caller must be local or present an admin or an active agent X509-SVID.
    fn get_federated_bundle("GetFederatedBundle")(GetFederatedBundleRequest) -> (BundleType);

    /// Batch creates one or more federated bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_create_federated_bundle("BatchCreateFederatedBundle")(BatchCreateFederatedBundleRequest) -> (BatchCreateFederatedBundleResponse);

    /// Batch updates one or more federated bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_update_federated_bundle("BatchUpdateFederatedBundle")(BatchUpdateFederatedBundleRequest) -> (BatchUpdateFederatedBundleResponse);

    /// Batch upserts one or more federated bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_set_federated_bundle("BatchSetFederatedBundle")(BatchSetFederatedBundleRequest) -> (BatchSetFederatedBundleResponse);

    /// Batch deletes one or more federated bundles.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_delete_federated_bundle("BatchDeleteFederatedBundle")(BatchDeleteFederatedBundleRequest) -> (BatchDeleteFederatedBundleResponse);
}
