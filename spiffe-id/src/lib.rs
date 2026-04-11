#![no_std]
extern crate alloc;

mod error;
mod id;
mod path;
#[cfg(feature = "serde")]
mod serde_support;
mod trust_domain;

pub use crate::{
    error::{SpiffeIdError, TrustDomainError},
    id::SpiffeId,
    path::Path,
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

#[inline]
const fn validate_path(path: &[u8]) -> Result<(), SpiffeIdError> {
    if path.is_empty() || path[0] != b'/' {
        return Err(SpiffeIdError::PathSeparator);
    }

    // 2.2. Path
    if path[path.len() - 1] == b'/' {
        return Err(SpiffeIdError::TrailingSlash);
    }

    // 2.2. Path
    let mut i = 0;
    while i < path.len() {
        if !validate_path_charset(path[i]) {
            return Err(SpiffeIdError::Character);
        }
        i += 1;
    }

    // 2.2. Path
    let mut segment_start = 1;
    let mut j = 1;
    while j <= path.len() {
        if j == path.len() || path[j] == b'/' {
            let segment_len = j - segment_start;
            if segment_len == 0 {
                return Err(SpiffeIdError::EmptySegment);
            }

            if segment_len == 1 && path[segment_start] == b'.' {
                return Err(SpiffeIdError::DotSegment);
            }

            if segment_len == 2 && path[segment_start] == b'.' && path[segment_start + 1] == b'.' {
                return Err(SpiffeIdError::DotSegment);
            }

            segment_start = j + 1;
        }

        j += 1;
    }

    Ok(())
}
