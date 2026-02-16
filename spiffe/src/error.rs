use core::fmt::{Display, Formatter, Result as FmtResult};

use spiffe_id::{SpiffeIdError, TrustDomainError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpiffeError {
    #[error("invalid SPIFFE ID: {0}")]
    SpiffeId(#[from] SpiffeIdError),

    #[error("invalid trust domain: {0}")]
    TrustDomain(#[from] TrustDomainError),

    #[error("JWT SVID is invalid")]
    InvalidJwtSvid,

    #[error("JWT bundle is invalid")]
    InvalidJwtBundle,

    #[error("invalid DER data")]
    InvalidDer(#[from] InvalidDerError),
}

#[derive(Error, Debug)]
pub struct InvalidDerError;

impl Display for InvalidDerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("invalid DER data")
    }
}
