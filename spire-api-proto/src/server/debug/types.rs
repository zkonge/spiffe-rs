use prost::Message;

use crate::SpiffeId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetInfoRequest {}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetInfoResponseCert {
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    #[prost(int64, tag = "2")]
    pub expires_at: i64,

    #[prost(string, tag = "3")]
    pub subject: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct GetInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub svid_chain: Vec<GetInfoResponseCert>,

    #[prost(int32, tag = "2")]
    pub uptime: i32,

    #[prost(int32, tag = "3")]
    pub agents_count: i32,

    #[prost(int32, tag = "4")]
    pub federated_bundles_count: i32,

    #[prost(int32, tag = "5")]
    pub entries_count: i32,
}
