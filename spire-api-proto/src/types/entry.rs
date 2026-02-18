use prost::Message;

use super::{Selector, SpiffeId};

#[derive(Clone, PartialEq, Message)]
pub struct Entry {
    /// Globally unique ID for the entry.
    #[prost(string, tag = "1")]
    pub id: String,

    /// The SPIFFE ID of the identity described by this entry.
    #[prost(message, optional, tag = "2")]
    pub spiffe_id: Option<SpiffeId>,

    /// Who the entry is delegated to.
    #[prost(message, optional, tag = "3")]
    pub parent_id: Option<SpiffeId>,

    /// The selectors which identify which entities match this entry.
    #[prost(message, repeated, tag = "4")]
    pub selectors: Vec<Selector>,

    /// The time to live for X509-SVID identities issued for this entry (in seconds).
    #[prost(int32, tag = "5")]
    pub x509_svid_ttl: i32,

    /// The names of trust domains the identity described by this entry federates with.
    #[prost(string, repeated, tag = "6")]
    pub federates_with: Vec<String>,

    /// Whether or not the identity described by this entry is an administrative workload.
    #[prost(bool, tag = "7")]
    pub admin: bool,

    /// Whether or not the identity described by this entry represents a downstream SPIRE server.
    #[prost(bool, tag = "8")]
    pub downstream: bool,

    /// When the entry expires (seconds since Unix epoch).
    #[prost(int64, tag = "9")]
    pub expires_at: i64,

    /// A list of DNS names associated with the identity described by this entry.
    #[prost(string, repeated, tag = "10")]
    pub dns_names: Vec<String>,

    /// Revision number is bumped every time the entry is updated.
    #[prost(int64, tag = "11")]
    pub revision_number: i64,

    /// Determines if the issued identity is exportable to a store.
    #[prost(bool, tag = "12")]
    pub store_svid: bool,

    /// The time to live for JWT-SVID identities issued for this entry (in seconds).
    #[prost(int32, tag = "13")]
    pub jwt_svid_ttl: i32,

    /// An operator-specified hint to guide workload identity selection.
    #[prost(string, tag = "14")]
    pub hint: String,

    /// When the entry was created (seconds since Unix epoch).
    #[prost(int64, tag = "15")]
    pub created_at: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct EntryMask {
    /// spiffe_id field mask
    #[prost(bool, tag = "2")]
    pub spiffe_id: bool,

    /// parent_id field mask
    #[prost(bool, tag = "3")]
    pub parent_id: bool,

    /// selectors field mask
    #[prost(bool, tag = "4")]
    pub selectors: bool,

    /// x509_svid_ttl field mask
    #[prost(bool, tag = "5")]
    pub x509_svid_ttl: bool,

    /// federates_with field mask
    #[prost(bool, tag = "6")]
    pub federates_with: bool,

    /// admin field mask
    #[prost(bool, tag = "7")]
    pub admin: bool,

    /// downstream field mask
    #[prost(bool, tag = "8")]
    pub downstream: bool,

    /// expires_at field mask
    #[prost(bool, tag = "9")]
    pub expires_at: bool,

    /// dns_names field mask
    #[prost(bool, tag = "10")]
    pub dns_names: bool,

    /// revision_number field mask
    #[prost(bool, tag = "11")]
    pub revision_number: bool,

    /// store_svid field mask
    #[prost(bool, tag = "12")]
    pub store_svid: bool,

    /// jwt_svid_ttl field mask
    #[prost(bool, tag = "13")]
    pub jwt_svid_ttl: bool,

    /// hint field mask
    #[prost(bool, tag = "14")]
    pub hint: bool,

    /// created_at field mask
    #[prost(bool, tag = "15")]
    pub created_at: bool,
}
