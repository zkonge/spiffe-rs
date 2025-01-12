use core::fmt::{Display, Formatter, Result as FmtResult};

use spiffe_id::SpiffeIdError;
use thiserror::Error;

#[cfg(feature = "client")]
#[derive(Error, Debug)]
pub enum SpiffeWorkloadClientError {
    #[error("workload API error: {0}")]
    SpiffeError(#[from] SpiffeError),

    #[error("gRPC status: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("STD error: {0}")]
    StdError(#[from] crate::StdError),
}

#[derive(Error, Debug)]
pub enum SpiffeError {
    #[error("invalid SPIFFE ID: {0}")]
    InvalidSpiffeId(#[from] SpiffeIdError),

    #[error("major part of SVID is empty")]
    EmptySvid,

    #[error("invalid DER data")]
    InvalidDerData(#[from] InvalidDerDataError),
}

#[derive(Error, Debug)]
pub struct InvalidDerDataError;

impl Display for InvalidDerDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("invalid DER data")
    }
}
