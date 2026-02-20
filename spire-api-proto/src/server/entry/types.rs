use prost::Message;

use crate::{Entry, EntryMask};

pub mod count_entries_response {
    use prost::Message;

    use crate::{FederatesWithMatch, SelectorMatch, SpiffeId};

    #[derive(Clone, PartialEq, Message)]
    pub struct Filter {
        #[prost(message, optional, tag = "1")]
        pub by_spiffe_id: Option<SpiffeId>,

        #[prost(message, optional, tag = "2")]
        pub by_parent_id: Option<SpiffeId>,

        #[prost(message, optional, tag = "3")]
        pub by_selectors: Option<SelectorMatch>,

        #[prost(message, optional, tag = "4")]
        pub by_federates_with: Option<FederatesWithMatch>,

        #[prost(string, optional, tag = "5")]
        pub by_hint: Option<String>,

        #[prost(bool, optional, tag = "6")]
        pub by_downstream: Option<bool>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CountEntriesRequest {
    /// Filters the entries returned in the response.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<count_entries_response::Filter>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountEntriesResponse {
    #[prost(int32, tag = "1")]
    pub count: i32,
}

pub mod list_entries_request {
    use prost::Message;

    use crate::{FederatesWithMatch, SelectorMatch, SpiffeId};

    #[derive(Clone, PartialEq, Message)]
    pub struct Filter {
        #[prost(message, optional, tag = "1")]
        pub by_spiffe_id: Option<SpiffeId>,

        #[prost(message, optional, tag = "2")]
        pub by_parent_id: Option<SpiffeId>,

        #[prost(message, optional, tag = "3")]
        pub by_selectors: Option<SelectorMatch>,

        #[prost(message, optional, tag = "4")]
        pub by_federates_with: Option<FederatesWithMatch>,

        #[prost(string, optional, tag = "5")]
        pub by_hint: Option<String>,

        #[prost(bool, optional, tag = "6")]
        pub by_downstream: Option<bool>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct ListEntriesRequest {
    /// Filters the entries returned in the response.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<list_entries_request::Filter>,

    /// An output mask indicating the entry fields set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,

    /// The maximum number of results to return. The server may further
    /// constrain this value, or if zero, choose its own.
    #[prost(int32, tag = "3")]
    pub page_size: i32,

    /// The next_page_token value returned from a previous request, if any.
    #[prost(string, tag = "4")]
    pub page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ListEntriesResponse {
    /// The list of entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<Entry>,

    /// The page token for the next request. Empty if there are no more results.
    /// This field should be checked by clients even when a page_size was not
    /// requested, since the server may choose its own (see page_size).
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetEntryRequest {
    /// Required. ID of the entry to get.
    #[prost(string, tag = "1")]
    pub id: String,

    /// An output mask indicating the entry fields set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateEntryRequest {
    /// The entries to be created. If no entry ID is provided, one will be
    /// generated.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<Entry>,

    /// An output mask indicating the entry fields set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,
}

pub mod batch_create_entry_response {
    use prost::Message;

    use crate::{Entry, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the entry. If status code will be
        /// ALREADY_EXISTS if a similar entry already exists. An entry is
        /// similar if it has the same spiffe_id, parent_id, and selectors.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The entry that was created (.e.g status code is OK) or that already
        /// exists (i.e. status code is ALREADY_EXISTS).
        //
        /// If the status code is any other value, this field will not be set.
        #[prost(message, optional, tag = "2")]
        pub entry: Option<Entry>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateEntryResponse {
    /// Result for each entry in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_create_entry_response::Result>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateEntryRequest {
    /// The entries to be updated.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<Entry>,

    /// An input mask indicating what entry fields should be updated.
    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<EntryMask>,

    /// An output mask indicating what entry fields are set in the response.
    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<EntryMask>,
}

pub mod batch_update_entry_response {
    use prost::Message;

    use crate::{Entry, Status};

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the entry.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The entry that was updated. If the status is OK, it will be the
        /// entry that was updated. If the status is any other value, this field
        /// will not be set.
        #[prost(message, optional, tag = "2")]
        pub entry: Option<Entry>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateEntryResponse {
    /// Result for each entry in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_update_entry_response::Result>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct BatchDeleteEntryRequest {
    /// Entry IDs to delete.
    #[prost(string, repeated, tag = "1")]
    pub ids: Vec<String>,
}

pub mod batch_delete_entry_response {
    use prost::Message;

    use crate::Status;

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The status of creating the entry.
        #[prost(message, optional, tag = "1")]
        pub status: Option<Status>,

        /// The ID of the entry that was deleted.
        #[prost(string, tag = "2")]
        pub id: String,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteEntryResponse {
    /// Result for each entry ID in the request (order is maintained).
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<batch_delete_entry_response::Result>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GetAuthorizedEntriesRequest {
    /// An output mask indicating which fields are set in the response.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<EntryMask>,
}

/// Authorized entries response.
#[derive(Clone, PartialEq, Message)]
pub struct GetAuthorizedEntriesResponse {
    /// The authorized entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<Entry>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SyncAuthorizedEntriesRequest {
    /// An output mask indicating which fields are set in the response.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<EntryMask>,

    /// IDs of the entries to fetch in full. Sent by the client in response to
    /// a sparse entry.
    #[prost(string, repeated, tag = "2")]
    pub ids: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SyncAuthorizedEntriesResponse {
    /// The revisions of the authorized entries. This field is set when the
    /// authorized entry list meets or exceeds the server-determined page size.
    /// Callers use it to determine which entries are new/updated that they then
    /// request on the stream.
    /// See SyncAuthorizedEntries for details.
    #[prost(message, repeated, tag = "1")]
    pub entry_revisions: Vec<EntryRevision>,

    /// The authorized entries. This field is set either 1) on the initial
    /// response if the number of authorized entries is less than the page size
    /// or 2) in response to the caller requesting the entries after determining
    /// they need to details based on entry revisions provided in a previous
    /// response.
    /// See SyncAuthorizedEntries for details.
    #[prost(message, repeated, tag = "2")]
    pub entries: Vec<Entry>,

    /// Whether there are more entries to sync down in this response phase.
    #[prost(bool, tag = "3")]
    pub more: bool,
}

/// Sparse revision metadata for one entry.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct EntryRevision {
    /// The entry ID.
    #[prost(string, tag = "1")]
    pub id: String,

    /// The entry revision number.
    #[prost(int64, tag = "2")]
    pub revision_number: i64,

    /// When the entry was created (seconds since Unix epoch).
    #[prost(int64, tag = "3")]
    pub created_at: i64,
}
