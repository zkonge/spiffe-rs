//! This module contains the high-level wrapper for the SPIFFE Workload API types
//! and useful functions to work with them.

pub mod client;
mod der;
mod error;
#[cfg(feature = "jwt")]
mod jwt;
mod types;

#[cfg(feature = "unchecked-api")]
pub use self::der::spiffe_id_from_x509_svid_unchecked;
#[cfg(all(feature = "jwt", feature = "unchecked-api"))]
pub use self::jwt::spiffe_id_from_jwt_svid_unchecked;
pub use self::{
    der::{CertificateIter, split_certificates},
    error::{InvalidDerError, SpiffeError},
    types::{JwtSvid, X509Bundle, X509Svid},
};

type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;
