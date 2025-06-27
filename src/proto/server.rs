use std::{
    convert::Infallible,
    future::Future,
    sync::Arc,
    task::{Context, Poll},
};

use futures_util::Stream;
use http::{HeaderValue, Response as HttpResponse};
use http_body::Body;
use prost::Message;
use tonic::{
    Request, Response, Status,
    body::Body as TonicBody,
    codec::{CompressionEncoding, EnabledCompressionEncodings, ProstCodec},
    server::{Grpc, NamedService},
    service::{Interceptor, interceptor::InterceptedService},
};
use tower_service::Service;

use super::{
    JwtBundlesRequest, JwtBundlesResponse, JwtSvidRequest, JwtSvidResponse, SPIFFE_METADATA_KEY,
    SPIFFE_METADATA_VALUE, ValidateJwtSvidRequest, ValidateJwtSvidResponse, X509BundlesRequest,
    X509BundlesResponse, X509SvidRequest, X509SvidResponse,
};
use crate::StdError;

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

impl<T: SpiffeWorkloadApi> SpiffeWorkloadApiServer<T> {
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

    pub fn into_service<B>(
        self,
    ) -> impl Service<http::Request<B>, Response = HttpResponse<TonicBody>>
    where
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        SvcFn(move |req: http::Request<B>| {
            let server_impl = self.clone();
            async move {
                if req
                    .headers()
                    .get(SPIFFE_METADATA_KEY)
                    .map(HeaderValue::as_bytes)
                    != Some(SPIFFE_METADATA_VALUE.as_bytes())
                {
                    let resp = Status::invalid_argument("security header missing from request")
                        .into_http();

                    return Ok::<_, Infallible>(resp);
                }

                let mut inner = Some(server_impl.inner.clone());

                enum Dispatcher<T1, T2, T3, T4, T5> {
                    FetchX509Svid(SvcFn<T1>),
                    FetchX509Bundles(SvcFn<T2>),
                    FetchJwtSvid(SvcFn<T3>),
                    FetchJwtBundles(SvcFn<T4>),
                    ValidateJwtSvid(SvcFn<T5>),
                    Unimplemented,
                }

                let svc = match req.uri().path().strip_prefix("/SpiffeWorkloadAPI/") {
                    Some("FetchX509SVID") => {
                        Dispatcher::FetchX509Svid(SvcFn(move |req: Request<X509SvidRequest>| {
                            let inner = inner.take().expect("only once");
                            async move { T::fetch_x509_svid(&inner, req).await }
                        }))
                    }
                    Some("FetchX509Bundles") => Dispatcher::FetchX509Bundles(SvcFn(
                        move |req: Request<X509BundlesRequest>| {
                            let inner = inner.take().expect("only once");
                            async move { T::fetch_x509_bundles(&inner, req).await }
                        },
                    )),
                    Some("FetchJWTSVID") => {
                        Dispatcher::FetchJwtSvid(SvcFn(move |req: Request<JwtSvidRequest>| {
                            let inner = inner.take().expect("only once");
                            async move { T::fetch_jwt_svid(&inner, req).await }
                        }))
                    }
                    Some("FetchJWTBundles") => Dispatcher::FetchJwtBundles(SvcFn(
                        move |req: Request<JwtBundlesRequest>| {
                            let inner = inner.take().expect("only once");
                            async move { T::fetch_jwt_bundles(&inner, req).await }
                        },
                    )),
                    Some("ValidateJWTSVID") => Dispatcher::ValidateJwtSvid(SvcFn(
                        move |req: Request<ValidateJwtSvidRequest>| {
                            let inner = inner.take().expect("only once");
                            async move { T::validate_jwt_svid(&inner, req).await }
                        },
                    )),
                    _ => Dispatcher::Unimplemented,
                };

                let resp = match svc {
                    Dispatcher::FetchX509Svid(svc) => {
                        server_impl.make_grpc().server_streaming(svc, req).await
                    }
                    Dispatcher::FetchX509Bundles(svc) => {
                        server_impl.make_grpc().server_streaming(svc, req).await
                    }
                    Dispatcher::FetchJwtSvid(svc) => server_impl.make_grpc().unary(svc, req).await,
                    Dispatcher::FetchJwtBundles(svc) => {
                        server_impl.make_grpc().server_streaming(svc, req).await
                    }
                    Dispatcher::ValidateJwtSvid(svc) => {
                        server_impl.make_grpc().unary(svc, req).await
                    }
                    Dispatcher::Unimplemented => {
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
                        response
                    }
                };

                Ok(resp)
            }
        })
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

pub struct SvcFn<F>(F);

impl<F, Fut, ReqTy, RespTy, E> Service<ReqTy> for SvcFn<F>
where
    F: FnMut(ReqTy) -> Fut,
    Fut: Future<Output = Result<RespTy, E>>,
{
    type Response = RespTy;
    type Error = E;
    type Future = Fut;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ReqTy) -> Self::Future {
        (self.0)(req)
    }
}
