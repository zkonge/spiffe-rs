use std::{convert::Infallible, future::Future, sync::Arc};

use futures_util::Stream;
use http::{Request as HttpRequest, Response as HttpResponse};
use http_body::Body;
use prost::Message;
use tonic::{
    Request, Response, Result,
    body::Body as TonicBody,
    server::{Grpc, NamedService},
    service::{Interceptor, interceptor::InterceptedService},
};
use tonic_prost::ProstCodec;
use tower_service::Service;

use super::{
    FetchJwtSvidsRequest, FetchJwtSvidsResponse, SubscribeToJwtBundlesRequest,
    SubscribeToJwtBundlesResponse, SubscribeToX509BundlesRequest, SubscribeToX509BundlesResponse,
    SubscribeToX509SvidsRequest, SubscribeToX509SvidsResponse,
};
use crate::{
    StdError,
    service::{SvcFn, unimplemented},
};

pub trait DelegatedIdentity: Send + Sync + 'static {
    /// Server streaming response type for the SubscribeToX509SVIDs method.
    type SubscribeToX509SVIDsStream: Stream<Item = Result<SubscribeToX509SvidsResponse>> + Send;

    /// Subscribe to get X.509-SVIDs for workloads that match the given selectors.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_x509_svids(
        &self,
        req: Request<SubscribeToX509SvidsRequest>,
    ) -> impl Future<Output = Result<Response<Self::SubscribeToX509SVIDsStream>>> + Send;

    /// Server streaming response type for the SubscribeToX509Bundles method.
    type SubscribeToX509BundlesStream: Stream<Item = Result<SubscribeToX509BundlesResponse>> + Send;

    /// Subscribe to get local and all federated bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_x509_bundles(
        &self,
        req: Request<SubscribeToX509BundlesRequest>,
    ) -> impl Future<Output = Result<Response<Self::SubscribeToX509BundlesStream>>> + Send;

    /// Fetch JWT-SVIDs for workloads that match the given selectors, and
    /// for the requested audience.
    fn fetch_jwt_svids(
        &self,
        req: Request<FetchJwtSvidsRequest>,
    ) -> impl Future<Output = Result<Response<FetchJwtSvidsResponse>>> + Send;

    /// Server streaming response type for the SubscribeToJWTBundles method.
    type SubscribeToJWTBundlesStream: Stream<Item = Result<SubscribeToJwtBundlesResponse>> + Send;

    /// Subscribe to get local and all federated JWKS bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    fn subscribe_to_jwt_bundles(
        &self,
        req: Request<SubscribeToJwtBundlesRequest>,
    ) -> impl Future<Output = Result<Response<Self::SubscribeToJWTBundlesStream>>> + Send;
}

/// The delegatedIdentity service provides an interface to get the SVIDs of other
/// workloads on the host. This service is intended for use cases where a process
/// (different than the workload one) should access the workload's SVID to
/// perform actions on behalf of the workload. One example of is using a single
/// node instance of Envoy that upgrades TCP connections for different processes
/// running in such a node.
///
/// The caller must be local and its identity must be listed in the allowed
/// clients on the spire-agent configuration.
#[derive(Debug)]
pub struct DelegatedIdentityServer<T> {
    inner: Arc<T>,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}

impl<T: DelegatedIdentity> DelegatedIdentityServer<T> {
    pub fn from_arc(inner: Arc<T>) -> Self {
        Self {
            inner,
            max_decoding_message_size: None,
            max_encoding_message_size: None,
        }
    }

    pub fn with_interceptor<F>(inner: Arc<T>, interceptor: F) -> InterceptedService<Self, F>
    where
        F: Interceptor,
    {
        InterceptedService::new(Self::from_arc(inner), interceptor)
    }

    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }

    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.max_encoding_message_size = Some(limit);
        self
    }

    #[inline]
    #[must_use]
    fn grpc<U, V>(&self) -> Grpc<ProstCodec<U, V>>
    where
        U: Message + 'static,
        V: Message + Default + 'static,
    {
        Grpc::new(ProstCodec::new()).apply_max_message_size_config(
            self.max_decoding_message_size,
            self.max_encoding_message_size,
        )
    }

    #[must_use]
    pub fn into_service<B>(
        self,
    ) -> impl Service<
        HttpRequest<B>,
        Response = HttpResponse<TonicBody>,
        Error = Infallible,
        Future = impl Future<Output = Result<HttpResponse<TonicBody>, Infallible>> + Send,
    > + Clone
    where
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        SvcFn(move |req: HttpRequest<B>| {
            let server = self.clone();
            async move {
                let inner = &*server.inner;

                let resp = match req
                    .uri()
                    .path()
                    .strip_prefix("/spire.api.agent.delegatedidentity.v1.DelegatedIdentity/")
                {
                    Some("SubscribeToX509SVIDs") => {
                        let s = SvcFn(|req| T::subscribe_to_x509_svids(inner, req));
                        server.grpc().server_streaming(s, req).await
                    }
                    Some("SubscribeToX509Bundles") => {
                        let s = SvcFn(|req| T::subscribe_to_x509_bundles(inner, req));
                        server.grpc().server_streaming(s, req).await
                    }
                    Some("FetchJWTSVIDs") => {
                        let s = SvcFn(|req| T::fetch_jwt_svids(inner, req));
                        server.grpc().unary(s, req).await
                    }
                    Some("SubscribeToJWTBundles") => {
                        let s = SvcFn(|req| T::subscribe_to_jwt_bundles(inner, req));
                        server.grpc().server_streaming(s, req).await
                    }
                    _ => unimplemented(),
                };

                Ok(resp)
            }
        })
    }
}

impl<T> Clone for DelegatedIdentityServer<T> {
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Self {
            inner,
            max_decoding_message_size: self.max_decoding_message_size,
            max_encoding_message_size: self.max_encoding_message_size,
        }
    }
}

impl<T> NamedService for DelegatedIdentityServer<T> {
    const NAME: &'static str = "spire.api.agent.delegatedidentity.v1.DelegatedIdentity";
}
