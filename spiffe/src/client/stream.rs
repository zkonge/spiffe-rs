use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures_core::Stream;
use spiffe_id::TrustDomain;
use tonic::Streaming;

use super::types::{JwtBundlesContext, X509BundlesContext, X509SvidContext};

macro_rules! impl_stream {
    ($name:ident, $proto_ty:ty, $resp_ty:ty) => {
        pub struct $name(pub(super) Streaming<$proto_ty>);

        impl Stream for $name {
            type Item = $resp_ty;

            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                match ready!(Pin::new(&mut self.0).poll_next(cx)) {
                    Some(Ok(x)) => Poll::Ready(x.try_into().ok()),
                    _ => Poll::Ready(None),
                }
            }
        }
    };

    ($name:ident, $proto_ty:ty, $resp_ty:ty => $item_ty:ty) => {
        pub struct $name(pub(super) Streaming<$proto_ty>);

        impl Stream for $name {
            type Item = $item_ty;

            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                let Some(Ok(resp)) = ready!(Pin::new(&mut self.0).poll_next(cx)) else {
                    return Poll::Ready(None);
                };

                let Ok(resp) = <$resp_ty>::try_from(resp) else {
                    return Poll::Ready(None);
                };

                Poll::Ready(Some(resp.into()))
            }
        }
    };
}

impl_stream!(
    X509SvidContextStream,
    spiffe_proto::X509SvidResponse,
    X509SvidContext
);
impl_stream!(
    X509BundlesContextStream,
    spiffe_proto::X509BundlesResponse,
    X509BundlesContext
);
impl_stream!(
    JwtBundlesStream,
    spiffe_proto::JwtBundlesResponse,
    JwtBundlesContext => HashMap<TrustDomain<'static>, String>
);
