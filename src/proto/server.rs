use std::{
    convert::Infallible,
    future::{Future, ready},
    sync::Arc,
    task::{Context, Poll},
};

use futures_util::{Stream, future::BoxFuture};
use http::{HeaderValue, Response as HttpResponse};
use http_body::Body;
use prost::Message;
use tonic::{
    Request, Response, Status,
    body::Body as TonicBody,
    codec::{CompressionEncoding, EnabledCompressionEncodings, ProstCodec},
    server::{Grpc, NamedService, ServerStreamingService, UnaryService},
    service::{Interceptor, interceptor::InterceptedService},
};
use tower_service::Service;

use super::{
    JwtBundlesRequest, JwtBundlesResponse, JwtSvidRequest, JwtSvidResponse, SPIFFE_METADATA_KEY,
    SPIFFE_METADATA_VALUE, ValidateJwtSvidRequest, ValidateJwtSvidResponse, X509BundlesRequest,
    X509BundlesResponse, X509SvidRequest, X509SvidResponse,
};
use crate::StdError;

type BoxResultFuture<T, E> = BoxFuture<'static, Result<T, E>>;

pub trait SpiffeWorkloadApi: Send + Sync + 'static {
    /// Server streaming response type for the FetchX509SVID method.
    type FetchX509SvidStream: Stream<Item = Result<X509SvidResponse, Status>> + Send + 'static;

    /// Fetch X.509-SVIDs for all SPIFFE identities the workload is entitled to,
    /// as well as related information like trust bundles and CRLs. As this
    /// information changes, subsequent messages will be streamed from the
    /// server.
    fn fetch_x509_svid(
        &self,
        req: Request<X509SvidRequest>,
    ) -> impl Future<Output = Result<Response<Self::FetchX509SvidStream>, Status>> + Send;

    /// Server streaming response type for the FetchX509Bundles method.
    type FetchX509BundlesStream: Stream<Item = Result<X509BundlesResponse, Status>> + Send + 'static;

    /// Fetch trust bundles and CRLs. Useful for clients that only need to
    /// validate SVIDs without obtaining an SVID for themself. As this
    /// information changes, subsequent messages will be streamed from the
    /// server.
    fn fetch_x509_bundles(
        &self,
        req: Request<X509BundlesRequest>,
    ) -> impl Future<Output = Result<Response<Self::FetchX509BundlesStream>, Status>> + Send;

    /// Fetch JWT-SVIDs for all SPIFFE identities the workload is entitled to,
    /// for the requested audience. If an optional SPIFFE ID is requested, only
    /// the JWT-SVID for that SPIFFE ID is returned.
    fn fetch_jwt_svid(
        &self,
        req: Request<JwtSvidRequest>,
    ) -> impl Future<Output = Result<Response<JwtSvidResponse>, Status>> + Send;

    /// Server streaming response type for the FetchJWTBundles method.
    type FetchJwtBundlesStream: Stream<Item = Result<JwtBundlesResponse, Status>> + Send + 'static;

    /// Fetches the JWT bundles, formatted as JWKS documents, keyed by the
    /// SPIFFE ID of the trust domain. As this information changes, subsequent
    /// messages will be streamed from the server.
    fn fetch_jwt_bundles(
        &self,
        req: Request<JwtBundlesRequest>,
    ) -> impl Future<Output = Result<Response<Self::FetchJwtBundlesStream>, Status>> + Send;

    /// Validates a JWT-SVID against the requested audience. Returns the SPIFFE
    /// ID of the JWT-SVID and JWT claims.
    fn validate_jwt_svid(
        &self,
        req: Request<ValidateJwtSvidRequest>,
    ) -> impl Future<Output = Result<Response<ValidateJwtSvidResponse>, Status>> + Send;
}

#[derive(Debug)]
pub struct SpiffeWorkloadApiServer<T> {
    inner: Arc<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}

impl<T> SpiffeWorkloadApiServer<T> {
    pub fn from_arc(inner: Arc<T>) -> Self {
        Self {
            inner,
            accept_compression_encodings: Default::default(),
            send_compression_encodings: Default::default(),
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

    /// Enable decompressing requests with the given encoding.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.accept_compression_encodings.enable(encoding);
        self
    }

    /// Compress responses with the given encoding, if the client supports it.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.send_compression_encodings.enable(encoding);
        self
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
    fn make_grpc<U, V>(&self) -> Grpc<ProstCodec<U, V>>
    where
        U: Message + 'static,
        V: Message + Default + 'static,
    {
        Grpc::new(ProstCodec::new())
            .apply_compression_config(
                self.accept_compression_encodings,
                self.send_compression_encodings,
            )
            .apply_max_message_size_config(
                self.max_decoding_message_size,
                self.max_encoding_message_size,
            )
    }
}

impl<T, B> Service<http::Request<B>> for SpiffeWorkloadApiServer<T>
where
    T: SpiffeWorkloadApi,
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
{
    type Response = HttpResponse<TonicBody>;
    type Error = Infallible;
    type Future = BoxResultFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        if req
            .headers()
            .get(SPIFFE_METADATA_KEY)
            .map(HeaderValue::as_bytes)
            != Some(SPIFFE_METADATA_VALUE.as_bytes())
        {
            let resp = Status::invalid_argument("security header missing from request").into_http();

            return Box::pin(ready(Ok(resp)));
        }

        match req.uri().path().strip_prefix("/SpiffeWorkloadAPI/") {
            Some("FetchX509SVID") => {
                struct FetchX509SvidService<T: SpiffeWorkloadApi>(pub Option<Arc<T>>);

                impl<T: SpiffeWorkloadApi> ServerStreamingService<X509SvidRequest> for FetchX509SvidService<T> {
                    type Response = X509SvidResponse;
                    type ResponseStream = T::FetchX509SvidStream;
                    type Future = BoxResultFuture<Response<Self::ResponseStream>, Status>;

                    fn call(&mut self, req: Request<X509SvidRequest>) -> Self::Future {
                        let inner = self.0.take().expect("only once");
                        Box::pin(async move { T::fetch_x509_svid(&inner, req).await })
                    }
                }

                let inner = Some(self.inner.clone());
                let mut grpc = self.make_grpc();

                Box::pin(async move {
                    Ok(grpc
                        .server_streaming(FetchX509SvidService(inner), req)
                        .await)
                })
            }
            Some("FetchX509Bundles") => {
                struct FetchX509BundlesService<T: SpiffeWorkloadApi>(pub Option<Arc<T>>);

                impl<T: SpiffeWorkloadApi> ServerStreamingService<X509BundlesRequest>
                    for FetchX509BundlesService<T>
                {
                    type Response = X509BundlesResponse;
                    type ResponseStream = T::FetchX509BundlesStream;
                    type Future = BoxResultFuture<Response<Self::ResponseStream>, Status>;

                    fn call(&mut self, req: Request<X509BundlesRequest>) -> Self::Future {
                        let inner = self.0.take().expect("only once");
                        Box::pin(async move { T::fetch_x509_bundles(&inner, req).await })
                    }
                }

                let inner = Some(self.inner.clone());
                let mut grpc = self.make_grpc();

                Box::pin(async move {
                    Ok(grpc
                        .server_streaming(FetchX509BundlesService(inner), req)
                        .await)
                })
            }
            Some("FetchJWTSVID") => {
                struct FetchJwtSvidService<T: SpiffeWorkloadApi>(pub Option<Arc<T>>);

                impl<T: SpiffeWorkloadApi> UnaryService<JwtSvidRequest> for FetchJwtSvidService<T> {
                    type Response = JwtSvidResponse;
                    type Future = BoxResultFuture<Response<Self::Response>, Status>;

                    fn call(&mut self, req: Request<JwtSvidRequest>) -> Self::Future {
                        let inner = self.0.take().expect("only once");
                        Box::pin(async move { T::fetch_jwt_svid(&inner, req).await })
                    }
                }

                let inner = Some(self.inner.clone());
                let mut grpc = self.make_grpc();

                Box::pin(async move { Ok(grpc.unary(FetchJwtSvidService(inner), req).await) })
            }
            Some("FetchJWTBundles") => {
                struct FetchJwtBundlesService<T: SpiffeWorkloadApi>(pub Option<Arc<T>>);

                impl<T: SpiffeWorkloadApi> ServerStreamingService<JwtBundlesRequest> for FetchJwtBundlesService<T> {
                    type Response = JwtBundlesResponse;
                    type ResponseStream = T::FetchJwtBundlesStream;
                    type Future = BoxResultFuture<Response<Self::ResponseStream>, Status>;

                    fn call(&mut self, req: Request<JwtBundlesRequest>) -> Self::Future {
                        let inner = self.0.take().expect("only once");
                        Box::pin(async move { T::fetch_jwt_bundles(&inner, req).await })
                    }
                }

                let inner = Some(self.inner.clone());
                let mut grpc = self.make_grpc();

                Box::pin(async move {
                    Ok(grpc
                        .server_streaming(FetchJwtBundlesService(inner), req)
                        .await)
                })
            }
            Some("ValidateJWTSVID") => {
                struct ValidateJwtSvidService<T: SpiffeWorkloadApi>(pub Option<Arc<T>>);

                impl<T: SpiffeWorkloadApi> UnaryService<ValidateJwtSvidRequest> for ValidateJwtSvidService<T> {
                    type Response = ValidateJwtSvidResponse;
                    type Future = BoxResultFuture<Response<Self::Response>, Status>;

                    fn call(&mut self, req: Request<ValidateJwtSvidRequest>) -> Self::Future {
                        let inner = self.0.take().expect("only once");
                        Box::pin(async move { T::validate_jwt_svid(&inner, req).await })
                    }
                }

                let inner = Some(self.inner.clone());
                let mut grpc = self.make_grpc();

                Box::pin(async move { Ok(grpc.unary(ValidateJwtSvidService(inner), req).await) })
            }
            _ => Box::pin({
                let mut response = HttpResponse::new(TonicBody::empty());
                let headers = response.headers_mut();
                headers.insert(
                    Status::GRPC_STATUS,
                    (tonic::Code::Unimplemented as i32).into(),
                );
                headers.insert(
                    http::header::CONTENT_TYPE,
                    tonic::metadata::GRPC_CONTENT_TYPE,
                );
                ready(Ok(response))
            }),
        }
    }
}

impl<T> Clone for SpiffeWorkloadApiServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            accept_compression_encodings: self.accept_compression_encodings,
            send_compression_encodings: self.send_compression_encodings,
            max_decoding_message_size: self.max_decoding_message_size,
            max_encoding_message_size: self.max_encoding_message_size,
        }
    }
}

impl<T> NamedService for SpiffeWorkloadApiServer<T> {
    const NAME: &'static str = "SpiffeWorkloadAPI";
}
