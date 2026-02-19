mod types;

pub use self::types::*;
use crate::{Entry as EntryType, macros::define_grpc};

define_grpc! {
    /// Registration entry management API.
    Entry,
    EntryClient,
    EntryServer,
    "spire.api.server.entry.v1.Entry",

    fn count_entries("CountEntries")(CountEntriesRequest) -> (CountEntriesResponse);

    fn list_entries("ListEntries")(ListEntriesRequest) -> (ListEntriesResponse);

    fn get_entry("GetEntry")(GetEntryRequest) -> (EntryType);

    fn batch_create_entry("BatchCreateEntry")(BatchCreateEntryRequest) -> (BatchCreateEntryResponse);

    fn batch_update_entry("BatchUpdateEntry")(BatchUpdateEntryRequest) -> (BatchUpdateEntryResponse);

    fn batch_delete_entry("BatchDeleteEntry")(BatchDeleteEntryRequest) -> (BatchDeleteEntryResponse);

    fn get_authorized_entries("GetAuthorizedEntries")(GetAuthorizedEntriesRequest) -> (GetAuthorizedEntriesResponse);

    fn sync_authorized_entries("SyncAuthorizedEntries")(stream SyncAuthorizedEntriesRequest) -> (stream SyncAuthorizedEntriesResponse) as SyncAuthorizedEntriesStream;
}
