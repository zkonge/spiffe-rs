#[macro_export]
macro_rules! define_client {
    (
        $(#[$client_attr:meta])*
        $client_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ );
        )*
    ) => {
        $(#[$client_attr])*
        #[derive(Debug, Clone)]
        pub struct $client_name<T> {
            inner: $crate::tonic::client::Grpc<T>,
        }

        impl<T> $client_name<T>
        where
            T: $crate::tonic::client::GrpcService<$crate::tonic::body::Body>,
            T::Error: Into<$crate::StdError>,
            T::ResponseBody: $crate::http_body::Body<Data = $crate::prost::bytes::Bytes> + Send + 'static,
            <T::ResponseBody as $crate::http_body::Body>::Error: Into<$crate::StdError> + Send,
        {
            pub fn with_origin(inner: T, origin: $crate::http::uri::Uri) -> Self {
                let inner = $crate::tonic::client::Grpc::with_origin(inner, origin);
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

            #[inline]
            fn ready(&mut self) -> impl Future<Output = $crate::tonic::Result<()>> + use<'_, T> {
                async {
                    match self.inner.ready().await {
                        Ok(()) => Ok(()),
                        Err(e) => {
                            Err($crate::tonic::Status::unknown(format!("Service was not ready: {}", e.into())))
                        }
                    }
                }
            }

            $(
                $crate::define_client! {
                    $service_name,
                    $(#[$method_attr])*
                    $method,
                    $method_name,
                    ($($request)+),
                    ($($response)+)
                }
            )*

        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::tonic::IntoRequest<$request_ty>,
        ) -> $crate::tonic::Result<$crate::tonic::Response<$response_ty>> {
            self.ready().await?;

            let mut req = request.into_request();
            req.extensions_mut()
                .insert($crate::tonic::GrpcMethod::new($service_name, $method_name));

            let path =
                $crate::http::uri::PathAndQuery::from_static(concat!("/", $service_name, "/", $method_name));

            self.inner
                .unary(req, path, $crate::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        ($request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::tonic::IntoRequest<$request_ty>,
        ) -> $crate::tonic::Result<$crate::tonic::Response<$crate::tonic::Streaming<$response_ty>>> {
            self.ready().await?;

            let mut req = request.into_request();
            req.extensions_mut()
                .insert($crate::tonic::GrpcMethod::new($service_name, $method_name));

            let path =
                $crate::http::uri::PathAndQuery::from_static(concat!("/", $service_name, "/", $method_name));

            self.inner
                .server_streaming(req, path, $crate::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::tonic::IntoStreamingRequest<Message = $request_ty>,
        ) -> $crate::tonic::Result<$crate::tonic::Response<$response_ty>> {
            self.ready().await?;

            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert($crate::tonic::GrpcMethod::new($service_name, $method_name));

            let path =
                $crate::http::uri::PathAndQuery::from_static(concat!("/", $service_name, "/", $method_name));

            self.inner
                .client_streaming(req, path, $crate::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::tonic::IntoStreamingRequest<Message = $request_ty>,
        ) -> $crate::tonic::Result<$crate::tonic::Response<$crate::tonic::Streaming<$response_ty>>> {
            self.ready().await?;

            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert($crate::tonic::GrpcMethod::new($service_name, $method_name));

            let path =
                $crate::http::uri::PathAndQuery::from_static(concat!("/", $service_name, "/", $method_name));

            self.inner
                .streaming(req, path, $crate::tonic_prost::ProstCodec::new())
                .await
        }
    };
}
