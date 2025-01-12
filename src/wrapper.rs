//! This module contains the high-level wrapper for the SPIFFE Workload API types
//! and useful functions to work with them.

#[cfg(feature = "client")]
mod client;
mod der;
mod error;
mod types;

#[cfg(feature = "client")]
pub use self::{client::SpiffeWorkloadClient, error::SpiffeWorkloadClientError};
pub use self::{
    der::{spiffe_id_from_x509_svid, split_certificates, CertificateIter},
    error::{InvalidDerDataError, SpiffeError},
    types::{JwtSvid, X509Svid},
};
