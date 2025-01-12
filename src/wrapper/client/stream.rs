use std::{
    pin::Pin,
    task::{ready, Context, Poll},
};

use futures_util::Stream;
use tonic::Streaming;

use crate::{wrapper::X509Svid, X509SvidResponse};

pub struct X509SvidStream(Streaming<X509SvidResponse>);

impl Stream for X509SvidStream {
    type Item = Vec<X509Svid>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let resp = match ready!(Pin::new(&mut self.get_mut().0).poll_next(cx)) {
            Some(Ok(s)) => s,
            Some(Err(_)) | None => return Poll::Ready(None),
        };

        Poll::Ready(
            resp.svids
                .into_iter()
                .map(X509Svid::try_from)
                .collect::<Result<Vec<_>, _>>()
                .ok(),
        )
    }
}

impl From<Streaming<X509SvidResponse>> for X509SvidStream {
    fn from(value: Streaming<X509SvidResponse>) -> Self {
        Self(value)
    }
}
