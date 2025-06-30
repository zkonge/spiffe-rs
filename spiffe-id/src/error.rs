use core::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub enum TrustDomainError {
    InvalidLength,
    Character,
}

impl Error for TrustDomainError {}

impl Display for TrustDomainError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use TrustDomainError::*;

        match self {
            InvalidLength => f.write_str("invalid length"),
            Character => f.write_str("invalid character"),
        }
    }
}

#[derive(Debug)]
pub enum SpiffeIdError {
    Scheme,
    Character,
    PathSeparator,
    TrailingSlash,
    TooLong,
    EmptySegment,
    DotSegment,
    TrustDomain(TrustDomainError),
}

impl Error for SpiffeIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use SpiffeIdError::*;

        match self {
            Scheme | Character | PathSeparator | TrailingSlash | TooLong | EmptySegment
            | DotSegment => None,
            TrustDomain(e) => Some(e),
        }
    }
}

impl Display for SpiffeIdError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use SpiffeIdError::*;

        match self {
            Scheme => f.write_str("invalid URL scheme"),
            Character => f.write_str("invalid character"),
            PathSeparator => f.write_str("invalid path separator"),
            TrailingSlash => f.write_str("trailing slash"),
            TooLong => f.write_str("too long"),
            EmptySegment => f.write_str("empty segment"),
            DotSegment => f.write_str("dot segment"),
            TrustDomain(e) => write!(f, "invalid trust domain: {e}"),
        }
    }
}
