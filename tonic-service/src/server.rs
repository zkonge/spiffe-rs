#[macro_export]
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
                $crate::define_server! {
                    @trait_item
                    $(#[$method_attr])*
                    $method,
                    ($($request)+),
                    ($($response)+)
                    $(as $stream_name)?
                }
            )*
        }

        $crate::define_server! {
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
            ) -> $crate::tonic::service::interceptor::InterceptedService<Self, F>
            where
                F: $crate::tonic::service::Interceptor,
            {
                $crate::tonic::service::interceptor::InterceptedService::new(
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
            fn grpc<U, V>(&self) -> $crate::tonic::server::Grpc<$crate::tonic_prost::ProstCodec<U, V>>
            where
                U: $crate::prost::Message + 'static,
                V: $crate::prost::Message + Default + 'static,
            {
                $crate::tonic::server::Grpc::new($crate::tonic_prost::ProstCodec::new()).apply_max_message_size_config(
                    self.max_decoding_message_size,
                    self.max_encoding_message_size,
                )
            }

            #[must_use]
            pub fn into_service<B>(
                self,
            ) -> impl $crate::tower_service::Service<
                $crate::http::Request<B>,
                Response = $crate::http::Response<$crate::tonic::body::Body>,
                Error = std::convert::Infallible,
                Future = impl Future<
                    Output = Result<$crate::http::Response<$crate::tonic::body::Body>, std::convert::Infallible>,
                > + Send,
            > + Clone
            where
                B: $crate::http_body::Body + Send + 'static,
                B::Error: Into<$crate::StdError> + Send + 'static,
            {
                $crate::SvcFn(move |req: $crate::http::Request<B>| {
                    let server = self.clone();
                    async move {
                        let inner = &*server.inner;

                        let resp = match req.uri().path().strip_prefix(concat!("/", $service_name, "/")) {
                            $(
                                Some($method_name) => {
                                    $crate::define_server! {
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
                            _ => $crate::grpc_unimplemented(),
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

        impl<T> $crate::tonic::server::NamedService for $server_name<T> {
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
        let s = $crate::SvcFn(|req| $inner.$method(req));
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
        let s = $crate::SvcFn(|req| $inner.$method(req));
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
        let s = $crate::SvcFn(|req| $inner.$method(req));
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
        let s = $crate::SvcFn(|req| $inner.$method(req));
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
            req: $crate::tonic::Request<$request_ty>,
        ) -> impl Future<Output = $crate::tonic::Result<$crate::tonic::Response<$response_ty>>> + Send;
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
            req: $crate::tonic::Request<$crate::tonic::Streaming<$request_ty>>,
        ) -> impl Future<Output = $crate::tonic::Result<$crate::tonic::Response<$response_ty>>> + Send;
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
        type $stream_name: $crate::futures_core::Stream<Item = $crate::tonic::Result<$response_ty>> + Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::tonic::Request<$request_ty>,
        ) -> impl Future<Output = $crate::tonic::Result<$crate::tonic::Response<Self::$stream_name>>> + Send;
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
        type $stream_name: $crate::futures_core::Stream<Item = $crate::tonic::Result<$response_ty>> + Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::tonic::Request<$crate::tonic::Streaming<$request_ty>>,
        ) -> impl Future<Output = $crate::tonic::Result<$crate::tonic::Response<Self::$stream_name>>> + Send;
    };
}
