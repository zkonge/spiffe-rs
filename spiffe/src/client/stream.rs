use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures_util::{Stream, StreamExt};
use spiffe_id::TrustDomain;
use tonic::Streaming;

use crate::client::types::{JwtBundlesContext, X509BundlesContext, X509SvidContext};

macro_rules! impl_stream {
    ($name:ident, $response_ty:ty, $proto_ty:ty) => {
        pub struct $name(pub(super) Streaming<$proto_ty>);

        impl Stream for $name {
            type Item = $response_ty;

            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                let response = match ready!(self.0.poll_next_unpin(cx)) {
                    Some(Ok(x)) => x,
                    _ => return Poll::Ready(None),
                };

                Poll::Ready(response.try_into().ok())
            }
        }
    };
}

impl_stream!(
    X509SvidContextStream,
    X509SvidContext,
    spiffe_proto::X509SvidResponse
);
impl_stream!(
    X509BundlesContextStream,
    X509BundlesContext,
    spiffe_proto::X509BundlesResponse
);

// This is a special case because the spiffe_proto is an external crate, we can't implement TryInto for it, so we have to do the conversion here.
pub struct JwtBundlesStream(pub(super) Streaming<spiffe_proto::JwtBundlesResponse>);

impl Stream for JwtBundlesStream {
    type Item = HashMap<TrustDomain<'static>, String>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let response = match ready!(self.0.poll_next_unpin(cx)) {
            Some(Ok(x)) => x,
            _ => return Poll::Ready(None),
        };

        let bundles: JwtBundlesContext = match response.try_into() {
            Ok(bundles) => bundles,
            _ => return Poll::Ready(None),
        };

        Poll::Ready(Some(bundles.bundles))
    }
}
