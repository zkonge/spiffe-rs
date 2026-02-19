use prost::Message;

use crate::{Entry as EntryType, EntryMask, FederatesWithMatch, SelectorMatch, SpiffeId, Status};

/// Request to count entries matching an optional filter.
#[derive(Clone, PartialEq, Message)]
pub struct CountEntriesRequest {
    /// Filters applied when counting entries.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<CountEntriesRequestFilter>,
}

/// Filter fields for counting entries.
#[derive(Clone, PartialEq, Message)]
pub struct CountEntriesRequestFilter {
    /// Match entries by SPIFFE ID.
    #[prost(message, optional, tag = "1")]
    pub by_spiffe_id: Option<SpiffeId>,

    /// Match entries by parent SPIFFE ID.
    #[prost(message, optional, tag = "2")]
    pub by_parent_id: Option<SpiffeId>,

    /// Match entries by selector expression.
    #[prost(message, optional, tag = "3")]
    pub by_selectors: Option<SelectorMatch>,

    /// Match entries by federates-with expression.
    #[prost(message, optional, tag = "4")]
    pub by_federates_with: Option<FederatesWithMatch>,

    /// Optional hint filter.
    #[prost(string, optional, tag = "5")]
    pub by_hint: Option<String>,

    /// If set, filters by downstream flag.
    #[prost(bool, optional, tag = "6")]
    pub by_downstream: Option<bool>,
}

/// Response carrying the number of matched entries.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountEntriesResponse {
    #[prost(int32, tag = "1")]
    pub count: i32,
}

/// Request to list entries with paging and optional filtering.
#[derive(Clone, PartialEq, Message)]
pub struct ListEntriesRequest {
    /// Filters applied when listing entries.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<ListEntriesRequestFilter>,

    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,

    /// Maximum number of items requested for this page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,

    /// Continuation token from a previous page.
    #[prost(string, tag = "4")]
    pub page_token: String,
}

/// Filter fields for listing entries.
#[derive(Clone, PartialEq, Message)]
pub struct ListEntriesRequestFilter {
    /// Match entries by SPIFFE ID.
    #[prost(message, optional, tag = "1")]
    pub by_spiffe_id: Option<SpiffeId>,

    /// Match entries by parent SPIFFE ID.
    #[prost(message, optional, tag = "2")]
    pub by_parent_id: Option<SpiffeId>,

    /// Match entries by selector expression.
    #[prost(message, optional, tag = "3")]
    pub by_selectors: Option<SelectorMatch>,

    /// Match entries by federates-with expression.
    #[prost(message, optional, tag = "4")]
    pub by_federates_with: Option<FederatesWithMatch>,

    /// Optional hint filter.
    #[prost(string, optional, tag = "5")]
    pub by_hint: Option<String>,

    /// If set, filters by downstream flag.
    #[prost(bool, optional, tag = "6")]
    pub by_downstream: Option<bool>,
}

/// Response page for list-entries operation.
#[derive(Clone, PartialEq, Message)]
pub struct ListEntriesResponse {
    /// Entries returned for this page.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<EntryType>,

    /// Continuation token for the next page, if any.
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

/// Request to fetch one entry by ID.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct GetEntryRequest {
    /// Entry identifier.
    #[prost(string, tag = "1")]
    pub id: String,

    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,
}

/// Request to create multiple entries.
#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateEntryRequest {
    /// Entries to create.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<EntryType>,

    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<EntryMask>,
}

/// Batch-create response with per-entry results.
#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateEntryResponse {
    /// Results in the same order as requested entries.
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchCreateEntryResponseResult>,
}

/// Per-entry creation result.
#[derive(Clone, PartialEq, Message)]
pub struct BatchCreateEntryResponseResult {
    /// Operation status for the corresponding entry.
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    /// Created or already-existing entry when applicable.
    #[prost(message, optional, tag = "2")]
    pub entry: Option<EntryType>,
}

/// Request to update multiple entries.
#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateEntryRequest {
    /// Entries to update.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<EntryType>,

    /// Field mask indicating which fields are updated.
    #[prost(message, optional, tag = "2")]
    pub input_mask: Option<EntryMask>,

    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "3")]
    pub output_mask: Option<EntryMask>,
}

/// Batch-update response with per-entry results.
#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateEntryResponse {
    /// Results in the same order as requested entries.
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchUpdateEntryResponseResult>,
}

/// Per-entry update result.
#[derive(Clone, PartialEq, Message)]
pub struct BatchUpdateEntryResponseResult {
    /// Operation status for the corresponding entry.
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    /// Updated entry when status is successful.
    #[prost(message, optional, tag = "2")]
    pub entry: Option<EntryType>,
}

/// Request to delete multiple entries by ID.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct BatchDeleteEntryRequest {
    /// Entry IDs to delete.
    #[prost(string, repeated, tag = "1")]
    pub ids: Vec<String>,
}

/// Batch-delete response with per-entry results.
#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteEntryResponse {
    /// Results in the same order as requested IDs.
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<BatchDeleteEntryResponseResult>,
}

/// Per-entry deletion result.
#[derive(Clone, PartialEq, Message)]
pub struct BatchDeleteEntryResponseResult {
    /// Operation status for the corresponding entry ID.
    #[prost(message, optional, tag = "1")]
    pub status: Option<Status>,

    /// Entry ID associated with this result.
    #[prost(string, tag = "2")]
    pub id: String,
}

/// Request to fetch entries authorized for the caller.
#[derive(Clone, PartialEq, Message)]
pub struct GetAuthorizedEntriesRequest {
    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<EntryMask>,
}

/// Authorized entries response.
#[derive(Clone, PartialEq, Message)]
pub struct GetAuthorizedEntriesResponse {
    /// Entries the caller is currently authorized for.
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<EntryType>,
}

/// One stream request in the authorized-entry sync flow.
#[derive(Clone, PartialEq, Message)]
pub struct SyncAuthorizedEntriesRequest {
    /// Field mask controlling which `Entry` fields are returned.
    #[prost(message, optional, tag = "1")]
    pub output_mask: Option<EntryMask>,

    /// Entry IDs requested in full by the client.
    #[prost(string, repeated, tag = "2")]
    pub ids: Vec<String>,
}

/// One stream response in the authorized-entry sync flow.
#[derive(Clone, PartialEq, Message)]
pub struct SyncAuthorizedEntriesResponse {
    /// Sparse revisions used by the client to detect updates.
    #[prost(message, repeated, tag = "1")]
    pub entry_revisions: Vec<EntryRevision>,

    /// Full entry payloads returned for this step.
    #[prost(message, repeated, tag = "2")]
    pub entries: Vec<EntryType>,

    /// Indicates whether more pages remain in the current phase.
    #[prost(bool, tag = "3")]
    pub more: bool,
}

/// Sparse revision metadata for one entry.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct EntryRevision {
    /// Entry identifier.
    #[prost(string, tag = "1")]
    pub id: String,

    /// Monotonic entry revision number.
    #[prost(int64, tag = "2")]
    pub revision_number: i64,

    /// Creation timestamp in seconds since Unix epoch.
    #[prost(int64, tag = "3")]
    pub created_at: i64,
}
