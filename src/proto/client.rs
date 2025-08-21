use std::future::Future;

use futures_util::TryFutureExt;
use http::uri::{PathAndQuery, Uri};
use http_body::Body;
use prost::bytes::Bytes;
use tonic::{
    GrpcMethod, IntoRequest, Request, Response, Result, Status, Streaming,
    body::Body as TonicBody,
    client::{Grpc, GrpcService},
    codec::CompressionEncoding,
    metadata::{MetadataKey, MetadataValue},
};
use tonic_prost::ProstCodec;

use super::{
    JwtBundlesRequest, JwtBundlesResponse, JwtSvidRequest, JwtSvidResponse, SPIFFE_METADATA_KEY,
    SPIFFE_METADATA_VALUE, ValidateJwtSvidRequest, ValidateJwtSvidResponse, X509BundlesRequest,
    X509BundlesResponse, X509SvidRequest, X509SvidResponse,
};
use crate::StdError;

#[inline]
fn make_request<T>(
    mut req: Request<T>,
    method: &'static str,
    path: &'static str,
) -> (Request<T>, PathAndQuery) {
    req.extensions_mut()
        .insert(GrpcMethod::new("SpiffeWorkloadAPI", method));
    req.metadata_mut().insert(
        MetadataKey::from_static(SPIFFE_METADATA_KEY),
        MetadataValue::from_static(SPIFFE_METADATA_VALUE),
    );

    (req, PathAndQuery::from_static(path))
}

#[derive(Clone, Debug)]
pub struct SpiffeWorkloadApiClient<T> {
    inner: Grpc<T>,
}

impl<T> SpiffeWorkloadApiClient<T>
where
    T: GrpcService<TonicBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        Self {
            inner: Grpc::with_origin(inner, origin),
        }
    }

    /// Compress requests with the given encoding.
    /// This requires the server to support it otherwise it might respond with an
    /// error.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.send_compressed(encoding);
        self
    }

    /// Enable decompressing responses.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.accept_compressed(encoding);
        self
    }

    /// Limits the maximum size of a decoded message.
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_decoding_message_size(limit);
        self
    }

    /// Limits the maximum size of an encoded message.
    /// Default: [`usize::MAX`]
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_encoding_message_size(limit);
        self
    }

    /// Fetch X.509-SVIDs for all SPIFFE identities the workload is entitled to,
    /// as well as related information like trust bundles and CRLs. As this
    /// information changes, subsequent messages will be streamed from the
    /// server.
    pub async fn fetch_x509_svid(
        &mut self,
        req: impl IntoRequest<X509SvidRequest>,
    ) -> Result<Response<Streaming<X509SvidResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "FetchX509SVID",
            "/SpiffeWorkloadAPI/FetchX509SVID",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    /// Fetch trust bundles and CRLs. Useful for clients that only need to
    /// validate SVIDs without obtaining an SVID for themself. As this
    /// information changes, subsequent messages will be streamed from the
    /// server.
    pub async fn fetch_x509_bundles(
        &mut self,
        req: impl IntoRequest<X509BundlesRequest>,
    ) -> Result<Response<Streaming<X509BundlesResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "FetchX509Bundles",
            "/SpiffeWorkloadAPI/FetchX509Bundles",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    /// Fetch JWT-SVIDs for all SPIFFE identities the workload is entitled to,
    /// for the requested audience. If an optional SPIFFE ID is requested, only
    /// the JWT-SVID for that SPIFFE ID is returned.
    pub async fn fetch_jwt_svid(
        &mut self,
        req: impl IntoRequest<JwtSvidRequest>,
    ) -> Result<Response<JwtSvidResponse>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "FetchJWTSVID",
            "/SpiffeWorkloadAPI/FetchJWTSVID",
        );

        self.inner.unary(req, path, ProstCodec::new()).await
    }

    /// Fetches the JWT bundles, formatted as JWKS documents, keyed by the
    /// SPIFFE ID of the trust domain. As this information changes, subsequent
    /// messages will be streamed from the server.
    pub async fn fetch_jwt_bundles(
        &mut self,
        req: impl IntoRequest<JwtBundlesRequest>,
    ) -> Result<Response<Streaming<JwtBundlesResponse>>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "FetchJWTBundles",
            "/SpiffeWorkloadAPI/FetchJWTBundles",
        );

        self.inner
            .server_streaming(req, path, ProstCodec::new())
            .await
    }

    /// Validates a JWT-SVID against the requested audience. Returns the SPIFFE
    /// ID of the JWT-SVID and JWT claims.
    pub async fn validate_jwt_svid(
        &mut self,
        req: impl IntoRequest<ValidateJwtSvidRequest>,
    ) -> Result<Response<ValidateJwtSvidResponse>> {
        self.ready().await?;

        let (req, path) = make_request(
            req.into_request(),
            "ValidateJWTSVID",
            "/SpiffeWorkloadAPI/ValidateJWTSVID",
        );

        self.inner.unary(req, path, ProstCodec::new()).await
    }

    #[inline]
    fn ready(&mut self) -> impl Future<Output = Result<()>> + use<'_, T> {
        self.inner
            .ready()
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))
    }
}
