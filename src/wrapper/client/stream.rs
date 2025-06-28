use core::task::Poll;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, ready},
};

use futures_util::{Stream, StreamExt};
use spiffe_id::TrustDomain;
use tonic::Streaming;

use crate::{
    proto,
    wrapper::client::types::{X509BundlesContext, X509SvidContext},
};

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
    proto::X509SvidResponse
);
impl_stream!(
    X509BundlesContextStream,
    X509BundlesContext,
    proto::X509BundlesResponse
);
impl_stream!(
    JwtBundlesStream,
    HashMap<TrustDomain<'static>, String>,
    proto::JwtBundlesResponse
);
