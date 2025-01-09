use bytes::Bytes;
use http::uri::{PathAndQuery, Uri};
use http_body::Body;
use tonic::{
    body::BoxBody,
    client::{Grpc, GrpcService},
    codec::CompressionEncoding,
    codec::ProstCodec,
    GrpcMethod, IntoRequest, Response, Status, Streaming,
};

use super::{
    JwtBundlesRequest, JwtBundlesResponse, JwtSvidRequest, JwtSvidResponse, StdError,
    ValidateJwtSvidRequest, ValidateJwtSvidResponse, X509BundlesRequest, X509BundlesResponse,
    X509SvidRequest, X509SvidResponse,
};

#[derive(Debug, Clone)]
pub struct SpiffeWorkloadApiClient<T> {
    inner: Grpc<T>,
}

impl<T> SpiffeWorkloadApiClient<T>
where
    T: GrpcService<BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        let inner = Grpc::with_origin(inner, origin);
        Self { inner }
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
    /// Default: `usize::MAX`
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
        request: impl IntoRequest<X509SvidRequest>,
    ) -> Result<Response<Streaming<X509SvidResponse>>, Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))?;

        let mut req = request.into_request();

        req.extensions_mut()
            .insert(GrpcMethod::new("SpiffeWorkloadAPI", "FetchX509Svid"));
        self.inner
            .server_streaming(
                req,
                PathAndQuery::from_static("/SpiffeWorkloadAPI/FetchX509Svid"),
                ProstCodec::default(),
            )
            .await
    }

    /// Fetch trust bundles and CRLs. Useful for clients that only need to
    /// validate SVIDs without obtaining an SVID for themself. As this
    /// information changes, subsequent messages will be streamed from the
    /// server.
    pub async fn fetch_x509_bundles(
        &mut self,
        request: impl IntoRequest<X509BundlesRequest>,
    ) -> Result<Response<Streaming<X509BundlesResponse>>, Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("SpiffeWorkloadAPI", "FetchX509Bundles"));
        self.inner
            .server_streaming(
                req,
                PathAndQuery::from_static("/SpiffeWorkloadAPI/FetchX509Bundles"),
                ProstCodec::new(),
            )
            .await
    }

    /// Fetch JWT-SVIDs for all SPIFFE identities the workload is entitled to,
    /// for the requested audience. If an optional SPIFFE ID is requested, only
    /// the JWT-SVID for that SPIFFE ID is returned.
    pub async fn fetch_jwtsvid(
        &mut self,
        request: impl IntoRequest<JwtSvidRequest>,
    ) -> Result<Response<JwtSvidResponse>, Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = ProstCodec::default();
        let path = PathAndQuery::from_static("/SpiffeWorkloadAPI/FetchJWTSVID");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("SpiffeWorkloadAPI", "FetchJWTSVID"));
        self.inner.unary(req, path, codec).await
    }

    /// Fetches the JWT bundles, formatted as JWKS documents, keyed by the
    /// SPIFFE ID of the trust domain. As this information changes, subsequent
    /// messages will be streamed from the server.
    pub async fn fetch_jwt_bundles(
        &mut self,
        request: impl IntoRequest<JwtBundlesRequest>,
    ) -> Result<Response<Streaming<JwtBundlesResponse>>, Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = ProstCodec::default();
        let path = PathAndQuery::from_static("/SpiffeWorkloadAPI/FetchJWTBundles");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("SpiffeWorkloadAPI", "FetchJWTBundles"));
        self.inner.server_streaming(req, path, codec).await
    }

    /// Validates a JWT-SVID against the requested audience. Returns the SPIFFE
    /// ID of the JWT-SVID and JWT claims.
    pub async fn validate_jwtsvid(
        &mut self,
        request: impl IntoRequest<ValidateJwtSvidRequest>,
    ) -> Result<Response<ValidateJwtSvidResponse>, Status> {
        self.inner
            .ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {}", e.into())))?;
        let codec = ProstCodec::default();
        let path = PathAndQuery::from_static("/SpiffeWorkloadAPI/ValidateJWTSVID");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("SpiffeWorkloadAPI", "ValidateJWTSVID"));
        self.inner.unary(req, path, codec).await
    }
}
