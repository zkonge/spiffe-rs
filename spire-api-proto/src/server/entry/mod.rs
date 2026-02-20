mod types;

pub use self::types::*;
use crate::{Entry as EntryType, macros::define_grpc};

define_grpc! {
    /// Manages registration entries stored by the SPIRE Server.
    Entry,
    EntryClient,
    EntryServer,
    "spire.api.server.entry.v1.Entry",

    /// Count entries.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn count_entries("CountEntries")(CountEntriesRequest) -> (CountEntriesResponse);

    /// Lists entries.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn list_entries("ListEntries")(ListEntriesRequest) -> (ListEntriesResponse);

    /// Gets an entry. If the entry does not exist, NOT_FOUND is returned.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn get_entry("GetEntry")(GetEntryRequest) -> (EntryType);

    /// Batch creates one or more entries.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_create_entry("BatchCreateEntry")(BatchCreateEntryRequest) -> (BatchCreateEntryResponse);

    /// Batch updates one or more entries.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_update_entry("BatchUpdateEntry")(BatchUpdateEntryRequest) -> (BatchUpdateEntryResponse);

    /// Batch deletes one or more entries.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn batch_delete_entry("BatchDeleteEntry")(BatchDeleteEntryRequest) -> (BatchDeleteEntryResponse);

    /// Gets the entries the caller is authorized for.
    //
    /// The caller must present an active agent X509-SVID. See the Agent
    /// AttestAgent/RenewAgent RPCs.
    fn get_authorized_entries("GetAuthorizedEntries")(GetAuthorizedEntriesRequest) -> (GetAuthorizedEntriesResponse);

    /// Syncs authorized entries down to the caller. The caller controls which
    /// entries the server sends down full details for. The flow is as follows:
    /// 1. Caller opens up sync stream
    /// 2. Server determines authorized entries for caller:
    ///    - If there are less entries than a server-determined page size, go to (5).
    ///    - Otherwise, go to (3).
    /// 3. Server pages entry revisions to the caller (contains the entry ID and
    ///    revision number). The "more" flag set for all pages but the last so
    ///    that the caller knows when the server is done.
    /// 4. Client determines which entries are new or updated (based on revision
    ///    number) and asks for them by sending a request with the IDs.
    /// 5. Server pages down entries to the caller for each ID identified in (4)
    ///    or every entry in (2) if the number of entries was less than the
    ///    server-determined page size. The "more" flag set for all pages but
    ///    the last so that the caller knows when the server is done.
    /// 6. Steps (4) and (5) are repeated until the caller has synced down the
    ///    details for all new/updated entries and closes the stream.
    fn sync_authorized_entries("SyncAuthorizedEntries")(stream SyncAuthorizedEntriesRequest) -> (stream SyncAuthorizedEntriesResponse) as SyncAuthorizedEntriesStream;
}
