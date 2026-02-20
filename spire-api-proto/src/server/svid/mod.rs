mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// SVID minting API.
    Svid,
    SvidClient,
    SvidServer,
    "spire.api.server.svid.v1.SVID",

    /// Mints a one-off X509-SVID outside of the normal node/workload
    /// registration process.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn mint_x509_svid("MintX509SVID")(MintX509SvidRequest) -> (MintX509SvidResponse);

    /// Mints a one-off JWT-SVID outside of the normal node/workload
    /// registration process.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn mint_jwt_svid("MintJWTSVID")(MintJwtSvidRequest) -> (MintJwtSvidResponse);

    /// Mints a one-off WIT-SVID outside of the normal node/workload
    /// registration process.
    //
    /// The caller must be local or present an admin X509-SVID.
    fn mint_wit_svid("MintWITSVID")(MintWitSvidRequest) -> (MintWitSvidResponse);

    /// Creates one or more X509-SVIDs from registration entries.
    //
    /// The caller must present an active agent X509-SVID that is authorized
    /// to mint the requested entries. See the Entry GetAuthorizedEntries RPC.
    fn batch_new_x509_svid("BatchNewX509SVID")(BatchNewX509SvidRequest) -> (BatchNewX509SvidResponse);

    /// Creates an JWT-SVID from a registration entry.
    //
    /// The caller must present an active agent X509-SVID that is authorized
    /// to mint the requested entry. See the Entry GetAuthorizedEntries RPC.
    fn new_jwt_svid("NewJWTSVID")(NewJwtSvidRequest) -> (NewJwtSvidResponse);

    /// Creates one or more WIT-SVIDs from registration entries.
    //
    /// The caller must present an active agent X509-SVID that is authorized
    /// to mint the requested entries. See the Entry GetAuthorizedEntries/SyncA RPC.
    fn batch_new_wit_svid("BatchNewWITSVID")(BatchNewWitSvidRequest) -> (BatchNewWitSvidResponse);

    /// Creates an X509 CA certificate appropriate for use by a downstream
    /// entity to mint X509-SVIDs.
    //
    /// The caller must present a downstream X509-SVID.
    fn new_downstream_x509_ca("NewDownstreamX509CA")(NewDownstreamX509CaRequest) -> (NewDownstreamX509CaResponse);
}
