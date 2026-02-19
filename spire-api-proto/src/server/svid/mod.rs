mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// SVID minting API.
    Svid,
    SvidClient,
    SvidServer,
    "spire.api.server.svid.v1.SVID",

    fn mint_x509_svid("MintX509SVID")(MintX509SvidRequest) -> (MintX509SvidResponse);

    fn mint_jwt_svid("MintJWTSVID")(MintJwtSvidRequest) -> (MintJwtSvidResponse);

    fn mint_wit_svid("MintWITSVID")(MintWitSvidRequest) -> (MintWitSvidResponse);

    fn batch_new_x509_svid("BatchNewX509SVID")(BatchNewX509SvidRequest) -> (BatchNewX509SvidResponse);

    fn new_jwt_svid("NewJWTSVID")(NewJwtSvidRequest) -> (NewJwtSvidResponse);

    fn batch_new_wit_svid("BatchNewWITSVID")(BatchNewWitSvidRequest) -> (BatchNewWitSvidResponse);

    fn new_downstream_x509_ca("NewDownstreamX509CA")(NewDownstreamX509CaRequest) -> (NewDownstreamX509CaResponse);
}
