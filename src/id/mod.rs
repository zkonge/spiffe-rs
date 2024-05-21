mod error;
#[cfg(feature = "serde")]
mod serde_support;

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

pub use error::SpiffeIdError;

const SPIFFE_SCHEMA: &str = "spiffe://";

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SpiffeId {
    id: Box<str>,
    path_offset: u16,
}

impl SpiffeId {
    pub fn parse(id: impl Into<Box<str>>) -> Result<Self, SpiffeIdError> {
        // following the SPIFFE ID standard
        // https://github.com/spiffe/spiffe/blob/67dc2b7d3f34f865be6d8bff20a7d6c6d29a4065/standards/SPIFFE-ID.md
        let id: Box<str> = id.into();

        // 2. SPIFFE Identity
        if !id.starts_with(SPIFFE_SCHEMA) {
            return Err(SpiffeIdError::InvalidSchema);
        }

        // 2.3. Maximum SPIFFE ID Length
        if id.len() > 2048 {
            return Err(SpiffeIdError::InvalidComponentLength);
        }

        // ASCII char would be ensured by the following check
        let bid = id
            .as_bytes()
            .get(SPIFFE_SCHEMA.len()..)
            .ok_or(SpiffeIdError::InvalidSchema)?;

        // 2. SPIFFE Identity
        let (td, path) = bid
            .iter()
            .position(|&x| x == b'/')
            .and_then(|offset| bid.split_at_checked(offset))
            .ok_or(SpiffeIdError::InvalidPathSeparator)?;

        // 2.1. Trust Domain
        // 2.3. Maximum SPIFFE ID Length
        if td.is_empty() || td.len() > 255 {
            return Err(SpiffeIdError::InvalidComponentLength);
        }

        // 2.1. Trust Domain
        if !td
            .iter()
            .all(|&c| matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_'))
        {
            return Err(SpiffeIdError::InvalidSpiffeChar);
        }

        // 2.2. Path
        if path.ends_with(b"/") {
            return Err(SpiffeIdError::TrailingSlash);
        }

        // 2.2. Path
        if !path.iter().all(
            |&c| matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'/'),
        ) {
            return Err(SpiffeIdError::InvalidSpiffeChar);
        }

        // 2.2. Path
        for segment in path.split(|&c| c == b'/').skip(1) {
            match segment {
                b"" => return Err(SpiffeIdError::EmptySegment),
                b"." | b".." => return Err(SpiffeIdError::DotSegment),
                _ => {}
            }
        }

        let path_offset = td.len() + SPIFFE_SCHEMA.len();

        Ok(Self {
            id,
            path_offset: path_offset as u16,
        })
    }

    #[inline]
    pub fn trust_domain(&self) -> &str {
        &self.id[SPIFFE_SCHEMA.len()..self.path_offset as usize]
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.id[self.path_offset as usize..]
    }
}

impl FromStr for SpiffeId {
    type Err = SpiffeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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
    use super::*;

    #[test]
    fn test_parse() {
        let id = SpiffeId::parse("spiffe://example.org/path").unwrap();
        assert_eq!(id.trust_domain(), "example.org");
        assert_eq!(id.path(), "/path");

        assert!(SpiffeId::parse("spiffe://example.org/").is_err());
        assert!(SpiffeId::parse("spiffe://example.org/path/").is_err());
    }

    #[test]
    fn test_to_string() {
        let id = SpiffeId::parse("spiffe://example.org/path").unwrap();
        assert_eq!(id.to_string(), "spiffe://example.org/path");
    }
}
