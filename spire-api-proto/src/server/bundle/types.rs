use prost::Message;

use crate::{Bundle as BundleType, BundleMask, JwtKey, WitKey, X509Certificate};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct CountBundlesRequest {}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountBundlesResponse {
    /// The total number of bundles, including the server bundle.
    #[prost(int32, tag = "1")]
    pub count: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct GetBundleRequest {
    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AppendBundleRequest {
    /// X.509 authorities to append.
    #[prost(message, repeated, tag = "1")]
    pub x509_authorities: Vec<X509Certificate>,

    /// JWT authorities to append.
    #[prost(message, repeated, tag = "2")]
    pub jwt_authorities: Vec<JwtKey>,

    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<BundleMask>,

    /// WIT authorities to append.
    #[prost(message, repeated, tag = "4")]
    pub wit_authorities: Vec<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishJwtAuthorityRequest {
    /// Required. The JWT authority to publish.
    #[prost(message, optional, tag = "1")]
    pub jwt_authority: Option<JwtKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishJwtAuthorityResponse {
    /// The JWT authorities for the trust domain.
    #[prost(message, repeated, tag = "1")]
    pub jwt_authorities: Vec<JwtKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishWitAuthorityRequest {
    /// Required. The WIT authority to publish.
    #[prost(message, optional, tag = "1")]
    pub wit_authority: Option<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PublishWitAuthorityResponse {
    /// The WIT authorities for the trust domain.
    #[prost(message, repeated, tag = "1")]
    pub wit_authorities: Vec<WitKey>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederatedBundlesRequest {
    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<BundleMask>,

    /// The maximum number of results to return. The server may further
    /// constrain this value, or if zero, choose its own.
    #[prost(int32, tag = "2")]
    pub page_size: i32,

    /// The next_page_token value returned from a previous request, if any.
    #[prost(string, tag = "3")]
    pub page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListFederatedBundlesResponse {
    /// The bundles.
    #[prost(message, repeated, tag = "1")]
    pub bundles: Vec<BundleType>,

    /// The page token for the next request. Empty if there are no more results.
    /// This field should be checked by clients even when a page_size was not
    /// requested, since the server may choose its own (see page_size).
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetFederatedBundleRequest {
    /// Required. The trust domain name of the bundle (e.g., "example.org").
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederatedBundleRequest {
    /// The bundles to be created.
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

pub mod batch_create_federated_bundle_response {
    use prost::Message;

    use crate::{Bundle, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the bundle.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The bundle that was created. This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub bundle: Option<Bundle>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateFederatedBundleResponse {
    /// Result for each bundle in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_create_federated_bundle_response::Result>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederatedBundleRequest {
    /// The bundles to be updated.
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    /// An input mask indicating which bundle fields should be updated.
    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<BundleMask>,

    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<BundleMask>,
}

pub mod batch_update_federated_bundle_response {
    use prost::Message;

    use crate::{Bundle, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of updating the bundle.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The bundle that was updated. This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub bundle: Option<Bundle>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateFederatedBundleResponse {
    /// Result for each bundle in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_update_federated_bundle_response::Result>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchSetFederatedBundleRequest {
    /// The bundles to be upserted.
    #[prost(message, repeated, tag = "1")]
    pub bundle: Vec<BundleType>,

    /// An output mask indicating which bundle fields are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<BundleMask>,
}

pub mod batch_set_federated_bundle_response {
    use prost::Message;

    use crate::{Bundle, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of upserting the bundle.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The bundle that was upserted. This will be set if the status is OK.
        #[prost(message, optional, tag = "2")]
        pub bundle: Option<Bundle>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchSetFederatedBundleResponse {
    /// Result for each bundle in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_set_federated_bundle_response::Result>,
}

pub mod batch_delete_federated_bundle_request {
    use prost::Enumeration;

    /// Mode controls the delete behavior if there are other records
    /// associated with the bundle (e.g. registration entries).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// RESTRICT prevents the bundle from being deleted in the presence of associated entries
        Restrict = 0,
        /// DELETE deletes the bundle and associated entries
        Delete = 1,
        /// DISSOCIATE deletes the bundle and dissociates associated entries
        Dissociate = 2,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederatedBundleRequest {
    /// The trust domain names (e.g., "example.org") of the bundles to delete.
    #[prost(string, repeated, tag = "1")]
    pub trust_domains: Vec<String>,

    /// The deletion mode selected
    #[prost(enumeration = "batch_delete_federated_bundle_request::Mode", tag = "2")]
    pub mode: i32,
}

pub mod batch_delete_federated_bundle_response {
    use prost::Message;

    use crate::Status;

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of deleting the bundle.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The trust domain name (e.g., "example.org") of the bundle that was
        /// deleted.
        #[prost(string, tag = "2")]
        pub trust_domain: String,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteFederatedBundleResponse {
    /// Result for each trust domain name in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_delete_federated_bundle_response::Result>,
}
