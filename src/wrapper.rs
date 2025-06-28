//! This module contains the high-level wrapper for the SPIFFE Workload API types
//! and useful functions to work with them.

#[cfg(feature = "client")]
pub mod client;
mod der;
mod error;
mod types;

pub use self::{
    der::{CertificateIter, spiffe_id_from_x509_svid, split_certificates},
    error::{InvalidDerDataError, SpiffeError},
    types::{JwtSvid, X509Bundle, X509Svid},
};
