use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetInfoRequest {}

pub mod get_info_response {
    use prost::{Message, bytes::Bytes};

    use crate::SpiffeId;

    #[derive(Clone, PartialEq, Message)]
    pub struct Cert {
        /// Certificate SPIFFE ID
        #[prost(message, optional, tag = "1")]
        pub id: Option<SpiffeId>,

        /// Expiration time
        #[prost(bytes, tag = "2")]
        pub cert: Bytes,

        /// Subject
        #[prost(bytes, tag = "3")]
        pub key: Bytes,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct GetInfoResponse {
    /// Server SVID chain
    #[prost(message, repeated, tag = "1")]
    pub svid_chain: Vec<get_info_response::Cert>,

    /// Server uptime in seconds
    #[prost(int32, tag = "2")]
    pub uptime: i32,

    /// Amount of registered agents
    #[prost(int32, tag = "3")]
    pub agents_count: i32,

    /// Amount of federated bundles
    #[prost(int32, tag = "4")]
    pub federated_bundles_count: i32,

    /// Amount of registration entries on database
    #[prost(int32, tag = "5")]
    pub entries_count: i32,
}
