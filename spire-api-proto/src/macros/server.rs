macro_rules! define_server {
    (
        $(#[$server_attr:meta])*
        $trait_name:ident,
        $server_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ ) $(as $stream_name:ident)?;
        )*
    ) => {
        pub trait $trait_name: Send + Sync + 'static {
            $(
                crate::macros::define_server! {
                    @trait_item
                    $(#[$method_attr])*
                    $method,
                    ($($request)+),
                    ($($response)+)
                    $(as $stream_name)?
                }
            )*
        }

        crate::macros::define_server! {
            @server
            $(#[$server_attr])*
            $server_name,
            $trait_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+);
            )*
        }
    };

    (
        @server
        $(#[$server_attr:meta])*
        $server_name:ident,
        $trait_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ );
        )*
    ) => {
        $(#[$server_attr])*
        #[derive(Debug)]
        pub struct $server_name<T> {
            inner: std::sync::Arc<T>,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }

        impl<T: $trait_name> $server_name<T> {
            pub fn from_arc(inner: std::sync::Arc<T>) -> Self {
                Self {
                    inner,
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }

            pub fn with_interceptor<F>(
                inner: std::sync::Arc<T>,
                interceptor: F,
            ) -> tonic::service::interceptor::InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                tonic::service::interceptor::InterceptedService::new(
                    Self::from_arc(inner),
                    interceptor,
                )
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
            fn grpc<U, V>(&self) -> tonic::server::Grpc<tonic_prost::ProstCodec<U, V>>
            where
                U: prost::Message + 'static,
                V: prost::Message + Default + 'static,
            {
                tonic::server::Grpc::new(tonic_prost::ProstCodec::new()).apply_max_message_size_config(
                    self.max_decoding_message_size,
                    self.max_encoding_message_size,
                )
            }

            #[must_use]
            pub fn into_service<B>(
                self,
            ) -> impl tower_service::Service<
                http::Request<B>,
                Response = http::Response<tonic::body::Body>,
                Error = std::convert::Infallible,
                Future = impl Future<
                    Output = Result<http::Response<tonic::body::Body>, std::convert::Infallible>,
                > + Send,
            > + Clone
            where
                B: http_body::Body + Send + 'static,
                B::Error: Into<crate::StdError> + Send + 'static,
            {
                crate::service::SvcFn(move |req: http::Request<B>| {
                    let server = self.clone();
                    async move {
                        let inner = &*server.inner;

                        let resp = match req.uri().path().strip_prefix(concat!("/", $service_name, "/")) {
                            $(
                                Some($method_name) => {
                                    crate::macros::define_server! {
                                        @route
                                        server,
                                        inner,
                                        req,
                                        $trait_name,
                                        $method,
                                        ($($request)+),
                                        ($($response)+)
                                    }
                                }
                            )*
                            _ => crate::service::unimplemented(),
                        };

                        Ok(resp)
                    }
                })
            }
        }

        impl<T> Clone for $server_name<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }

        impl<T> tonic::server::NamedService for $server_name<T> {
            const NAME: &'static str = $service_name;
        }
    };

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {{
        let s = crate::service::SvcFn(|req| $inner.$method(req));
        $server.grpc().unary(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        ($request_ty:ty),
        (stream $response_ty:ty)
    ) => {{
        let s = crate::service::SvcFn(|req| $inner.$method(req));
        $server.grpc().server_streaming(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {{
        let s = crate::service::SvcFn(|req| $inner.$method(req));
        $server.grpc().client_streaming(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
    ) => {{
        let s = crate::service::SvcFn(|req| $inner.$method(req));
        $server.grpc().streaming(s, $req).await
    }};

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: tonic::Request<$request_ty>,
        ) -> impl Future<Output = tonic::Result<tonic::Response<$response_ty>>> + Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: tonic::Request<tonic::Streaming<$request_ty>>,
        ) -> impl Future<Output = tonic::Result<tonic::Response<$response_ty>>> + Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        ($request_ty:ty),
        (stream $response_ty:ty)
        as $stream_name:ident
    ) => {
        $(#[$method_attr])*
        type $stream_name: futures_core::Stream<Item = tonic::Result<$response_ty>> + Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: tonic::Request<$request_ty>,
        ) -> impl Future<Output = tonic::Result<tonic::Response<Self::$stream_name>>> + Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
        as $stream_name:ident
    ) => {
        $(#[$method_attr])*
        type $stream_name: futures_core::Stream<Item = tonic::Result<$response_ty>> + Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: tonic::Request<tonic::Streaming<$request_ty>>,
        ) -> impl Future<Output = tonic::Result<tonic::Response<Self::$stream_name>>> + Send;
    };
}

pub(crate) use define_server;
