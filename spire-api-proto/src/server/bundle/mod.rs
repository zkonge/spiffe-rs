mod types;

pub use self::types::*;
use crate::{Bundle as BundleType, macros::define_grpc};

define_grpc! {
    /// Bundle management API for SPIRE Server.
    Bundle,
    BundleClient,
    BundleServer,
    "spire.api.server.bundle.v1.Bundle",

    fn count_bundles("CountBundles")(CountBundlesRequest) -> (CountBundlesResponse);

    fn get_bundle("GetBundle")(GetBundleRequest) -> (BundleType);

    fn append_bundle("AppendBundle")(AppendBundleRequest) -> (BundleType);

    fn publish_jwt_authority("PublishJWTAuthority")(PublishJwtAuthorityRequest) -> (PublishJwtAuthorityResponse);

    fn publish_wit_authority("PublishWITAuthority")(PublishWitAuthorityRequest) -> (PublishWitAuthorityResponse);

    fn list_federated_bundles("ListFederatedBundles")(ListFederatedBundlesRequest) -> (ListFederatedBundlesResponse);

    fn get_federated_bundle("GetFederatedBundle")(GetFederatedBundleRequest) -> (BundleType);

    fn batch_create_federated_bundle("BatchCreateFederatedBundle")(BatchCreateFederatedBundleRequest) -> (BatchCreateFederatedBundleResponse);

    fn batch_update_federated_bundle("BatchUpdateFederatedBundle")(BatchUpdateFederatedBundleRequest) -> (BatchUpdateFederatedBundleResponse);

    fn batch_set_federated_bundle("BatchSetFederatedBundle")(BatchSetFederatedBundleRequest) -> (BatchSetFederatedBundleResponse);

    fn batch_delete_federated_bundle("BatchDeleteFederatedBundle")(BatchDeleteFederatedBundleRequest) -> (BatchDeleteFederatedBundleResponse);
}
