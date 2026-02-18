use prost::Message;

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct JoinToken {
    /// The value of the token.
    #[prost(string, tag = "1")]
    pub value: String,

    /// The token expiration (seconds since Unix epoch).
    #[prost(int64, tag = "2")]
    pub expires_at: i64,
}
