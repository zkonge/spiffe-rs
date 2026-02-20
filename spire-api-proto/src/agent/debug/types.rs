use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetInfoRequest {}

pub mod get_info_response {
    use prost::Message;

    use crate::SpiffeId;

    #[derive(Clone, PartialEq, Eq, Hash, Message)]
    pub struct Cert {
        /// Cerfificate SPIFFE ID
        #[prost(message, optional, tag = "1")]
        pub id: Option<SpiffeId>,

        /// Expiration time
        #[prost(int64, tag = "2")]
        pub expires_at: i64,

        /// Subject
        #[prost(string, tag = "3")]
        pub subject: String,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct GetInfoResponse {
    /// Agent SVID chain
    #[prost(message, repeated, tag = "1")]
    pub svid_chain: Vec<get_info_response::Cert>,

    /// Agent uptime in seconds
    #[prost(int32, tag = "2")]
    pub uptime: i32,

    /// Number of SVIDs cached in memory
    /// Deprecated in favor of cached_x509_svids_count, cached_jwt_svids_count, and cached_svidstore_x509_svids_count
    #[prost(int32, tag = "3")]
    pub svids_count: i32,

    /// last successful sync with server (in seconds since unix epoch)
    #[prost(int64, tag = "4")]
    pub last_sync_success: i64,

    /// Number of X.509-SVIDs in Agent primary in-memory cache
    #[prost(int32, tag = "5")]
    pub cached_x509_svids_count: i32,

    /// Number of JWT-SVIDs in Agent primary in-memory cache
    #[prost(int32, tag = "6")]
    pub cached_jwt_svids_count: i32,

    /// Number of X.509-SVIDs in Agent svidstore in-memory cache
    #[prost(int32, tag = "7")]
    pub cached_svidstore_x509_svids_count: i32,
}
