use prost::Message;

/// A SPIFFE ID, consisting of the trust domain name and a path portions of
/// the SPIFFE ID URI.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct SpiffeId {
    /// Trust domain portion the SPIFFE ID (e.g. "example.org")
    #[prost(string, tag = "1")]
    pub trust_domain: String,

    /// The path component of the SPIFFE ID (e.g. "/foo/bar/baz"). The path
    /// SHOULD have a leading slash. Consumers MUST normalize the path before
    /// making any sort of comparison between IDs.
    #[prost(string, tag = "2")]
    pub path: String,
}
