//! Represents a SPIFFE ID, which is a standardized identifier for workloads in a SPIFFE-compliant system.
//!
//! A SPIFFE ID is composed of a scheme (`spiffe://`), a trust domain, and a path. This struct
//! encapsulates the full SPIFFE ID as a boxed string slice, and provides methods to access the
//! trust domain and path components.
//!
//! # Examples
//!
//! ```
//! use spiffe_id::{SpiffeId, TrustDomain};
//!
//! let id = SpiffeId::new("spiffe://example.org/service").unwrap();
//! assert_eq!(id.trust_domain(), TrustDomain::const_new("example.org"));
//! assert_eq!(id.path(), "/service");
//! ```
//!
//! # References
//!
//! - [SPIFFE ID Standard](https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE-ID.md)

use alloc::{boxed::Box, string::String};
use core::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::{
    SPIFFE_SCHEME, SpiffeIdError, TrustDomain, validate_path_charset, validate_trust_domain,
};

/// A unique identifier for a workload within the SPIFFE ecosystem.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SpiffeId {
    id: Box<str>,
    path_offset: u16,
}

impl SpiffeId {
    /// Creates a new `SpiffeId` from the given identifier.
    ///
    /// It would be zero-copy if the input is already a [`Box<str>`] or could be convert into [`Box<str>`]
    /// without allocation.
    pub fn new(id: impl Into<Box<str>>) -> Result<Self, SpiffeIdError> {
        // following the SPIFFE ID standard
        // https://github.com/spiffe/spiffe/blob/67dc2b7d3f34f865be6d8bff20a7d6c6d29a4065/standards/SPIFFE-ID.md
        let id: Box<str> = id.into();

        // 2.3. Maximum SPIFFE ID Length
        if id.len() > 2048 {
            return Err(SpiffeIdError::InvalidComponentLength);
        }

        // 2. SPIFFE Identity
        let Some((SPIFFE_SCHEME, sid)) = id.split_at_checked(SPIFFE_SCHEME.len()) else {
            return Err(SpiffeIdError::InvalidScheme);
        };

        // ASCII char would be ensured by the following check
        let bid = sid.as_bytes();

        // 2. SPIFFE Identity
        let (td, path) = bid
            .iter()
            .position(|&x| x == b'/')
            .and_then(|offset| bid.split_at_checked(offset))
            .ok_or(SpiffeIdError::InvalidPathSeparator)?;

        validate_trust_domain(td)?;

        // 2.2. Path
        if path.ends_with(b"/") {
            return Err(SpiffeIdError::TrailingSlash);
        }

        // 2.2. Path
        if !path.iter().cloned().all(validate_path_charset) {
            return Err(SpiffeIdError::InvalidSpiffeIdCharacter);
        }

        // 2.2. Path
        for segment in path.split(|&c| c == b'/').skip(1) {
            match segment {
                b"" => return Err(SpiffeIdError::EmptySegment),
                b"." | b".." => return Err(SpiffeIdError::DotSegment),
                _ => {}
            }
        }

        let path_offset = td.len() + SPIFFE_SCHEME.len();

        Ok(Self {
            id,
            path_offset: path_offset as u16,
        })
    }

    /// Returns the trust domain component of the SPIFFE ID.
    #[inline]
    pub const fn trust_domain(&self) -> TrustDomain<'_> {
        let (leading, _path) = self.id.split_at(self.path_offset as usize);
        let (_scheme, trust_domain) = leading.split_at(SPIFFE_SCHEME.len());

        TrustDomain::new_unchecked(trust_domain)
    }

    /// Returns the path component of the SPIFFE ID.
    #[inline]
    pub const fn path(&self) -> &str {
        let (_, path) = self.id.split_at(self.path_offset as usize);

        path
    }

    /// Returns the full SPIFFE ID as a string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.id
    }
}

impl FromStr for SpiffeId {
    type Err = SpiffeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<SpiffeId> for Box<str> {
    fn from(id: SpiffeId) -> Self {
        id.id
    }
}

impl From<SpiffeId> for String {
    fn from(id: SpiffeId) -> Self {
        id.id.into()
    }
}

impl Debug for SpiffeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SpiffeId")
            .field("trust_domain", &self.trust_domain())
            .field("path", &self.path())
            .finish()
    }
}

impl Display for SpiffeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_parse() {
        let id = SpiffeId::new("spiffe://example.org/path").unwrap();
        assert_eq!(id.trust_domain(), TrustDomain::new("example.org").unwrap());
        assert_eq!(id.path(), "/path");

        assert!(SpiffeId::new("spiffe://example.org/").is_err());
        assert!(SpiffeId::new("spiffe://example.org/path/").is_err());
    }

    #[test]
    fn test_to_string() {
        let id = SpiffeId::new("spiffe://example.org/path").unwrap();
        assert_eq!(id.to_string(), "spiffe://example.org/path");
    }
}
