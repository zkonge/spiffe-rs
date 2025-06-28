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
    wrapper::client::types::{X509BundlesResponse, X509SvidResponse},
};

macro_rules! impl_stream {
    ($name:ident, $response_ty:ident) => {
        pub struct $name(pub(super) Streaming<proto::$response_ty>);

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

impl_stream!(X509SvidResponseStream, X509SvidResponse);
impl_stream!(X509BundlesResponseStream, X509BundlesResponse);

// JwtBundles stream is special because it returns a bare HashMap instead of a structured response type.
pub struct JwtBundlesStream(pub(super) Streaming<proto::JwtBundlesResponse>);

impl Stream for JwtBundlesStream {
    type Item = HashMap<TrustDomain<'static>, String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let response = match ready!(self.0.poll_next_unpin(cx)) {
            Some(Ok(x)) => x,
            _ => return Poll::Ready(None),
        };

        Poll::Ready(response.try_into().ok())
    }
}
