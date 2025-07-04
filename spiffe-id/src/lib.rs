#![no_std]
extern crate alloc;

mod error;
mod id;
#[cfg(feature = "serde")]
mod serde_support;
mod trust_domain;

pub use crate::{
    error::{SpiffeIdError, TrustDomainError},
    id::SpiffeId,
    trust_domain::TrustDomain,
};

const SPIFFE_SCHEME: &str = "spiffe://";

macro_rules! tri {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

pub(crate) use tri;

#[inline]
const fn validate_trust_domain_charset(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_')
}

#[inline]
const fn validate_path_charset(c: u8) -> bool {
    matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'/')
}

#[inline]
const fn validate_trust_domain(td: &[u8]) -> Result<(), TrustDomainError> {
    // 2.1. Trust Domain
    // 2.3. Maximum SPIFFE ID Length
    if td.is_empty() || td.len() > 255 {
        return Err(TrustDomainError::InvalidLength);
    }

    // 2.1. Trust Domain
    let mut i = 0;

    while i < td.len() {
        if !validate_trust_domain_charset(td[i]) {
            return Err(TrustDomainError::Character);
        }
        i += 1;
    }

    Ok(())
}
