use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpiffeIdError {
    #[error("invalid URL schema")]
    InvalidSchema,

    #[error("invalid SPIFFE ID character")]
    InvalidSpiffeIdCharacter,

    #[error("invalid path separator")]
    InvalidPathSeparator,

    #[error("trailing slash")]
    TrailingSlash,

    #[error("invalid SPIFFE ID component length")]
    InvalidComponentLength,

    #[error("empty segment")]
    EmptySegment,

    #[error("dot segment")]
    DotSegment,
}
