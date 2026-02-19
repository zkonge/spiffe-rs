mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// Local authority management API.
    LocalAuthority,
    LocalAuthorityClient,
    LocalAuthorityServer,
    "spire.api.server.localauthority.v1.LocalAuthority",

    fn get_jwt_authority_state("GetJWTAuthorityState")(GetJwtAuthorityStateRequest) -> (GetJwtAuthorityStateResponse);

    fn prepare_jwt_authority("PrepareJWTAuthority")(PrepareJwtAuthorityRequest) -> (PrepareJwtAuthorityResponse);

    fn activate_jwt_authority("ActivateJWTAuthority")(ActivateJwtAuthorityRequest) -> (ActivateJwtAuthorityResponse);

    fn taint_jwt_authority("TaintJWTAuthority")(TaintJwtAuthorityRequest) -> (TaintJwtAuthorityResponse);

    fn revoke_jwt_authority("RevokeJWTAuthority")(RevokeJwtAuthorityRequest) -> (RevokeJwtAuthorityResponse);

    fn get_x509_authority_state("GetX509AuthorityState")(GetX509AuthorityStateRequest) -> (GetX509AuthorityStateResponse);

    fn prepare_x509_authority("PrepareX509Authority")(PrepareX509AuthorityRequest) -> (PrepareX509AuthorityResponse);

    fn activate_x509_authority("ActivateX509Authority")(ActivateX509AuthorityRequest) -> (ActivateX509AuthorityResponse);

    fn taint_x509_authority("TaintX509Authority")(TaintX509AuthorityRequest) -> (TaintX509AuthorityResponse);

    fn taint_x509_upstream_authority("TaintX509UpstreamAuthority")(TaintX509UpstreamAuthorityRequest) -> (TaintX509UpstreamAuthorityResponse);

    fn revoke_x509_authority("RevokeX509Authority")(RevokeX509AuthorityRequest) -> (RevokeX509AuthorityResponse);

    fn revoke_x509_upstream_authority("RevokeX509UpstreamAuthority")(RevokeX509UpstreamAuthorityRequest) -> (RevokeX509UpstreamAuthorityResponse);

    fn get_wit_authority_state("GetWITAuthorityState")(GetWitAuthorityStateRequest) -> (GetWitAuthorityStateResponse);

    fn prepare_wit_authority("PrepareWITAuthority")(PrepareWitAuthorityRequest) -> (PrepareWitAuthorityResponse);

    fn activate_wit_authority("ActivateWITAuthority")(ActivateWitAuthorityRequest) -> (ActivateWitAuthorityResponse);

    fn taint_wit_authority("TaintWITAuthority")(TaintWitAuthorityRequest) -> (TaintWitAuthorityResponse);

    fn revoke_wit_authority("RevokeWITAuthority")(RevokeWitAuthorityRequest) -> (RevokeWitAuthorityResponse);
}
