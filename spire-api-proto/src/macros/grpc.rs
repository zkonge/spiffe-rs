macro_rules! define_grpc {
    (
        $(#[$grpc_attr:meta])*
        $trait_name:ident,
        $client_name:ident,
        $server_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ ) $(as $stream_name:ident)?;
        )*
    ) => {
        #[cfg(feature = "client")]
        crate::macros::define_client! {
            $(#[$grpc_attr])*
            $client_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+);
            )*
        }

        #[cfg(feature = "server")]
        crate::macros::define_server! {
            $(#[$grpc_attr])*
            $trait_name,
            $server_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+) $(as $stream_name)?;
            )*
        }
    };
}

pub(crate) use define_grpc;
