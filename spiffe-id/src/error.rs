use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrustDomainError {
    #[error("too long")]
    TooLong,

    #[error("invalid character")]
    Character,
}

#[derive(Error, Debug)]
pub enum SpiffeIdError {
    #[error("invalid URL scheme")]
    Scheme,

    #[error("invalid character")]
    Character,

    #[error("invalid path separator")]
    PathSeparator,

    #[error("trailing slash")]
    TrailingSlash,

    #[error("too long")]
    TooLong,

    #[error("empty segment")]
    EmptySegment,

    #[error("dot segment")]
    DotSegment,

    #[error("invalid trust domain: {0}")]
    TrustDomain(#[from] TrustDomainError),
}
