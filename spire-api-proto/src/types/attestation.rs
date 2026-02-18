use prost::{Message, bytes::Bytes};

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct AttestationData {
    /// The type of attestation data. This is typically the name of the plugin
    /// that produced that data.
    #[prost(string, tag = "1")]
    pub r#type: String,

    /// The attestation data payload.
    #[prost(bytes = "bytes", tag = "2")]
    pub payload: Bytes,
}
