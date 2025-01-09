mod client;
mod server;
mod types;

pub use client::SpiffeWorkloadApiClient;
pub use types::*;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

/*
pub mod spiffe_workload_api_server {
    use tonic::codegen::*;
    pub trait SpiffeWorkloadApi: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the FetchX509SVID method.
        type FetchX509SVIDStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::X509svidResponse, tonic::Status>,
            > + std::marker::Send
            + 'static;
        /// Fetch X.509-SVIDs for all SPIFFE identities the workload is entitled to,
        /// as well as related information like trust bundles and CRLs. As this
        /// information changes, subsequent messages will be streamed from the
        /// server.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn fetch_x509svid<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::X509svidRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<Self::FetchX509SVIDStream>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;

        /// Server streaming response type for the FetchX509Bundles method.
        type FetchX509BundlesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::X509BundlesResponse, tonic::Status>,
            > + std::marker::Send
            + 'static;
        /// Fetch trust bundles and CRLs. Useful for clients that only need to
        /// validate SVIDs without obtaining an SVID for themself. As this
        /// information changes, subsequent messages will be streamed from the
        /// server.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn fetch_x509_bundles<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::X509BundlesRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<Self::FetchX509BundlesStream>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;

        /// Fetch JWT-SVIDs for all SPIFFE identities the workload is entitled to,
        /// for the requested audience. If an optional SPIFFE ID is requested, only
        /// the JWT-SVID for that SPIFFE ID is returned.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn fetch_jwtsvid<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::JwtsvidRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::JwtsvidResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;

        /// Server streaming response type for the FetchJWTBundles method.
        type FetchJWTBundlesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::JwtBundlesResponse, tonic::Status>,
            > + std::marker::Send
            + 'static;
        /// Fetches the JWT bundles, formatted as JWKS documents, keyed by the
        /// SPIFFE ID of the trust domain. As this information changes, subsequent
        /// messages will be streamed from the server.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn fetch_jwt_bundles<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::JwtBundlesRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<Self::FetchJWTBundlesStream>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;

        /// Validates a JWT-SVID against the requested audience. Returns the SPIFFE
        /// ID of the JWT-SVID and JWT claims.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn validate_jwtsvid<'life0, 'async_trait>(
            &'life0 self,
            request: tonic::Request<super::ValidateJwtsvidRequest>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::ValidateJwtsvidResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
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
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
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

        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.

        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SpiffeWorkloadApiServer<T>
    where
        T: SpiffeWorkloadApi,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/SpiffeWorkloadAPI/FetchX509SVID" => {
                    #[allow(non_camel_case_types)]
                    struct FetchX509SVIDSvc<T: SpiffeWorkloadApi>(pub Arc<T>);

                    impl<T: SpiffeWorkloadApi>
                        tonic::server::ServerStreamingService<super::X509svidRequest>
                        for FetchX509SVIDSvc<T>
                    {
                        type Response = super::X509svidResponse;
                        type ResponseStream = T::FetchX509SVIDStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::X509svidRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SpiffeWorkloadApi>::fetch_x509svid(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchX509SVIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/SpiffeWorkloadAPI/FetchX509Bundles" => {
                    #[allow(non_camel_case_types)]
                    struct FetchX509BundlesSvc<T: SpiffeWorkloadApi>(pub Arc<T>);

                    impl<T: SpiffeWorkloadApi>
                        tonic::server::ServerStreamingService<super::X509BundlesRequest>
                        for FetchX509BundlesSvc<T>
                    {
                        type Response = super::X509BundlesResponse;
                        type ResponseStream = T::FetchX509BundlesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::X509BundlesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SpiffeWorkloadApi>::fetch_x509_bundles(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchX509BundlesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/SpiffeWorkloadAPI/FetchJWTSVID" => {
                    #[allow(non_camel_case_types)]
                    struct FetchJWTSVIDSvc<T: SpiffeWorkloadApi>(pub Arc<T>);

                    impl<T: SpiffeWorkloadApi> tonic::server::UnaryService<super::JwtsvidRequest>
                        for FetchJWTSVIDSvc<T>
                    {
                        type Response = super::JwtsvidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JwtsvidRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SpiffeWorkloadApi>::fetch_jwtsvid(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchJWTSVIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/SpiffeWorkloadAPI/FetchJWTBundles" => {
                    #[allow(non_camel_case_types)]
                    struct FetchJWTBundlesSvc<T: SpiffeWorkloadApi>(pub Arc<T>);

                    impl<T: SpiffeWorkloadApi>
                        tonic::server::ServerStreamingService<super::JwtBundlesRequest>
                        for FetchJWTBundlesSvc<T>
                    {
                        type Response = super::JwtBundlesResponse;
                        type ResponseStream = T::FetchJWTBundlesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JwtBundlesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SpiffeWorkloadApi>::fetch_jwt_bundles(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchJWTBundlesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/SpiffeWorkloadAPI/ValidateJWTSVID" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateJWTSVIDSvc<T: SpiffeWorkloadApi>(pub Arc<T>);

                    impl<T: SpiffeWorkloadApi>
                        tonic::server::UnaryService<super::ValidateJwtsvidRequest>
                        for ValidateJWTSVIDSvc<T>
                    {
                        type Response = super::ValidateJwtsvidResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidateJwtsvidRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SpiffeWorkloadApi>::validate_jwtsvid(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ValidateJWTSVIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for SpiffeWorkloadApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "SpiffeWorkloadAPI";
    impl<T> tonic::server::NamedService for SpiffeWorkloadApiServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
*/
