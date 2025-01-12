use std::{
    future::{poll_fn, ready, Future, Ready},
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{ready, Context, Poll},
};

use futures_util::{future::BoxFuture, FutureExt, TryFutureExt};
use http::{Request, Response};
use hyper::{
    body::Incoming,
    client::conn::http2::{handshake as h2_handshake, SendRequest},
    rt::Executor,
};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::UnixStream;
use tonic::{body::BoxBody, ConnectError};
use tower_service::Service;

use crate::StdError;

async fn connect_unix_socket(path: Arc<Path>) -> Result<SendRequest<BoxBody>, StdError> {
    let s = UnixStream::connect(path).map_ok(TokioIo::new).await?;
    let e = TokioExecutor::new();

    let (send_request, conn) = h2_handshake(e.clone(), s).await?;
    e.execute(conn);

    Ok(send_request)
}

enum State {
    Idle,
    Connecting(BoxFuture<'static, Result<SendRequest<BoxBody>, StdError>>),
    Connected(SendRequest<BoxBody>),
}

struct ConnectionService {
    unix_socket_path: Arc<Path>,
    state: State,
    error: Option<StdError>,
    has_connected: bool,
}

impl ConnectionService {
    pub fn new(unix_socket_path: Arc<Path>) -> Self {
        Self {
            unix_socket_path,
            state: State::Idle,
            error: None,
            has_connected: false,
        }
    }
}

impl Service<()> for ConnectionService {
    type Response = SendRequest<BoxBody>;
    type Error = StdError;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.error.is_some() {
            return Poll::Ready(Ok(()));
        }

        loop {
            match &mut self.state {
                State::Idle => {
                    let fut = connect_unix_socket(self.unix_socket_path.clone())
                        .map_err(|e| ConnectError(e).into())
                        .boxed();
                    self.state = State::Connecting(fut);
                    continue;
                }
                State::Connecting(f) => match ready!(Pin::new(f).poll(cx)) {
                    Ok(service) => {
                        self.state = State::Connected(service);
                    }
                    Err(e) => {
                        self.state = State::Idle;

                        if !self.has_connected {
                            return Poll::Ready(Err(e));
                        } else {
                            self.error = Some(e);
                            break;
                        }
                    }
                },
                State::Connected(inner) => {
                    self.has_connected = true;

                    match ready!(inner.poll_ready(cx)) {
                        Ok(()) => {
                            return Poll::Ready(Ok(()));
                        }
                        Err(_) => {
                            self.state = State::Idle;
                        }
                    }
                }
            }
        }

        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        if let Some(error) = self.error.take() {
            return ready(Err(error));
        }

        let State::Connected(client) = &mut self.state else {
            panic!("service not ready; poll_ready must be called first");
        };

        ready(Ok(client.clone()))
    }
}

#[derive(Clone)]
pub(super) struct UnixSpiffeClient(Arc<Mutex<ConnectionService>>);

impl UnixSpiffeClient {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self, StdError> {
        let mut svc = ConnectionService::new(path.as_ref().into());

        // ensure connection is established
        poll_fn(|cx| svc.poll_ready(cx)).await?;

        Ok(Self(Arc::new(svc.into())))
    }
}

impl Service<Request<BoxBody>> for UnixSpiffeClient {
    type Response = Response<Incoming>;
    type Error = StdError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0
            .lock()
            .expect("panic while poll_ready")
            .poll_ready(cx)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        match self
            .0
            .lock()
            .expect("panic while call")
            .call(())
            .into_inner()
        {
            Ok(mut c) => c.send_request(req).map_err(Into::into).boxed(),
            Err(e) => ready(Err(e)).boxed(),
        }
    }
}
