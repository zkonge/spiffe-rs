use std::{
    future::Future,
    task::{Context, Poll},
};

use http::{Response as HttpResponse, header::CONTENT_TYPE};
use tonic::{Code, Result, Status, body::Body as TonicBody, metadata::GRPC_CONTENT_TYPE};
use tower_service::Service;

#[derive(Clone)]
pub(crate) struct SvcFn<F>(pub F);

impl<F, Fut, ReqTy, RespTy, E> Service<ReqTy> for SvcFn<F>
where
    F: FnMut(ReqTy) -> Fut,
    Fut: Future<Output = Result<RespTy, E>>,
{
    type Response = RespTy;
    type Error = E;
    type Future = Fut;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ReqTy) -> Self::Future {
        (self.0)(req)
    }
}

pub(crate) fn unimplemented() -> HttpResponse<TonicBody> {
    let mut response = HttpResponse::new(TonicBody::empty());
    let headers = response.headers_mut();
    headers.insert(Status::GRPC_STATUS, (Code::Unimplemented as i32).into());
    headers.insert(CONTENT_TYPE, GRPC_CONTENT_TYPE);
    response
}
