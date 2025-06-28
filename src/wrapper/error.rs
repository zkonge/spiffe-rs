use core::fmt::{Display, Formatter, Result as FmtResult};

use spiffe_id::{SpiffeIdError, TrustDomainError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpiffeError {
    #[error("invalid SPIFFE ID: {0}")]
    SpiffeId(#[from] SpiffeIdError),

    #[error("invalid trust domain: {0}")]
    TrustDomain(#[from] TrustDomainError),

    #[error("major part of SVID is empty")]
    EmptySvid,

    #[error("JWT bundle is not valid UTF-8")]
    InvalidJwtBundle,

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
