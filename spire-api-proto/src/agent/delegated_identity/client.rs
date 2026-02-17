use std::future::Future;

use futures_util::TryFutureExt;
use http::uri::{PathAndQuery, Uri};
use http_body::Body;
use prost::bytes::Bytes;
use tonic::{
    GrpcMethod, IntoRequest, Request, Response, Result, Status, Streaming,
    body::Body as TonicBody,
    client::{Grpc, GrpcService},
};
use tonic_prost::ProstCodec;

use super::{
    FetchJwtSvidsRequest, FetchJwtSvidsResponse, SubscribeToJwtBundlesRequest,
    SubscribeToJwtBundlesResponse, SubscribeToX509BundlesRequest, SubscribeToX509BundlesResponse,
    SubscribeToX509SvidsRequest, SubscribeToX509SvidsResponse,
};
use crate::StdError;

#[inline]
fn make_request<T>(
    mut req: Request<T>,
    method: &'static str,
    path: &'static str,
) -> (Request<T>, PathAndQuery) {
    req.extensions_mut().insert(GrpcMethod::new(
        "spire.api.agent.delegatedidentity.v1.DelegatedIdentity",
        method,
    ));

    (req, PathAndQuery::from_static(path))
}

#[derive(Debug, Clone)]
pub struct DelegatedIdentityClient<T> {
    inner: Grpc<T>,
}

impl<T> DelegatedIdentityClient<T>
where
    T: GrpcService<TonicBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        let inner = Grpc::with_origin(inner, origin);
        Self { inner }
    }

    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_decoding_message_size(limit);
        self
    }

    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_encoding_message_size(limit);
        self
    }

    /// Subscribe to get X.509-SVIDs for workloads that match the given selectors.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    pub async fn subscribe_to_x509_svids(
        &mut self,
        req: impl IntoRequest<SubscribeToX509SvidsRequest>,
    ) -> Result<Response<Streaming<SubscribeToX509SvidsResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "SubscribeToX509SVIDs",
            "/spire.api.agent.delegatedidentity.v1.DelegatedIdentity/SubscribeToX509SVIDs",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    /// Subscribe to get local and all federated bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    pub async fn subscribe_to_x509_bundles(
        &mut self,
        request: impl IntoRequest<SubscribeToX509BundlesRequest>,
    ) -> Result<Response<Streaming<SubscribeToX509BundlesResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            request.into_request(),
            "SubscribeToX509Bundles",
            "/spire.api.agent.delegatedidentity.v1.DelegatedIdentity/SubscribeToX509Bundles",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    /// Fetch JWT-SVIDs for workloads that match the given selectors, and
    /// for the requested audience.
    pub async fn fetch_jwt_svids(
        &mut self,
        request: impl IntoRequest<FetchJwtSvidsRequest>,
    ) -> Result<Response<FetchJwtSvidsResponse>> {
        self.ready().await?;

        let (req, path) = make_request(
            request.into_request(),
            "FetchJWTSVIDs",
            "/spire.api.agent.delegatedidentity.v1.DelegatedIdentity/FetchJWTSVIDs",
        );

        self.inner.unary(req, path, ProstCodec::new()).await
    }

    /// Subscribe to get local and all federated JWKS bundles.
    /// The lifetime of the subscription aligns to the lifetime of the stream.
    pub async fn subscribe_to_jwt_bundles(
        &mut self,
        request: impl IntoRequest<SubscribeToJwtBundlesRequest>,
    ) -> Result<Response<Streaming<SubscribeToJwtBundlesResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            request.into_request(),
            "SubscribeToJWTBundles",
            "/spire.api.agent.delegatedidentity.v1.DelegatedIdentity/SubscribeToJWTBundles",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    #[inline]
    fn ready(&mut self) -> impl Future<Output = Result<()>> + use<'_, T> {
        self.inner
            .ready()
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))
    }
}
