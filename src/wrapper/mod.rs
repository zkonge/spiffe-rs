//! This module contains the high-level wrapper for the SPIFFE Workload API types
//! and useful functions to work with them.

mod error;
mod splitter;
mod types;

pub use self::{
    error::{InvalidDerDataError, InvalidSvidError},
    splitter::{split_certificates, CertificateIter},
    types::{JwtSvid, X509Svid},
};
