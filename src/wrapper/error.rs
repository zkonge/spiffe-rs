use std::fmt::{Display, Formatter, Result as FmtResult};

use spiffe_id::SpiffeIdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidSvidError {
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
