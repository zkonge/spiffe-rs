mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// The delegatedIdentity service provides an interface to get the SVIDs of other
    /// workloads on the host. This service is intended for use cases where a process
    /// (different than the workload one) should access the workload's SVID to
    /// perform actions on behalf of the workload. One example of is using a single
    /// node instance of Envoy that upgrades TCP connections for different processes
    /// running in such a node.
    ///
    /// The caller must be local and its identity must be listed in the allowed
    /// clients on the spire-agent configuration.
    DelegatedIdentity,
    DelegatedIdentityClient,
    DelegatedIdentityServer,
    "spire.api.agent.delegatedidentity.v1.DelegatedIdentity",

    /// Subscribe to get X.509-SVIDs for workloads that match the given selectors.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_x509_svids("SubscribeToX509SVIDs")(SubscribeToX509SvidsRequest) -> (stream SubscribeToX509SvidsResponse) as SubscribeToX509SvidsStream;

    /// Subscribe to get local and all federated bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_x509_bundles("SubscribeToX509Bundles")(SubscribeToX509BundlesRequest) -> (stream SubscribeToX509BundlesResponse) as SubscribeToX509BundlesStream;

    /// Fetch JWT-SVIDs for workloads that match the given selectors, and
    /// for the requested audience.
    fn fetch_jwt_svids("FetchJWTSVIDs")(FetchJwtSvidsRequest) -> (FetchJwtSvidsResponse);

    /// Subscribe to get local and all federated JWKS bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_jwt_bundles("SubscribeToJWTBundles")(SubscribeToJwtBundlesRequest) -> (stream SubscribeToJwtBundlesResponse) as SubscribeToJWTBundlesStream;
}
