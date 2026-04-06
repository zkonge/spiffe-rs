use std::{
    future::Future,
    task::{Context, Poll},
};

use http::{Response as HttpResponse, header::CONTENT_TYPE};
use tonic::{
    Code, Status, body::Body as TonicBody, metadata::GRPC_CONTENT_TYPE, server::NamedService,
};
use tower_service::Service;

#[derive(Clone)]
pub struct SvcFn<F>(pub F);

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

#[derive(Clone)]
pub struct NamedSvcFn<F, T: NamedService>(pub F, pub T);

impl<F, Fut, ReqTy, RespTy, E, T> Service<ReqTy> for NamedSvcFn<F, T>
where
    F: FnMut(ReqTy) -> Fut,
    Fut: Future<Output = Result<RespTy, E>>,
    T: NamedService,
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

impl<F, T: NamedService> NamedService for NamedSvcFn<F, T> {
    const NAME: &'static str = T::NAME;
}

pub fn grpc_unimplemented() -> HttpResponse<TonicBody> {
    let mut response = HttpResponse::new(TonicBody::empty());
    let headers = response.headers_mut();
    headers.insert(Status::GRPC_STATUS, (Code::Unimplemented as i32).into());
    headers.insert(CONTENT_TYPE, GRPC_CONTENT_TYPE);
    response
}
