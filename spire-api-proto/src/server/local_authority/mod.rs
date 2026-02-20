mod types;

pub use self::types::*;
use crate::macros::define_grpc;

define_grpc! {
    /// The LocalAuthority service provides a way to manage the signing keys (and
    /// related material) of the SPIRE Server exposing it.
    LocalAuthority,
    LocalAuthorityClient,
    LocalAuthorityServer,
    "spire.api.server.localauthority.v1.LocalAuthority",

    /// GetJWTAuthorityState returns the state of all locally configured
    /// JWT authorities.
    fn get_jwt_authority_state("GetJWTAuthorityState")(GetJwtAuthorityStateRequest) -> (GetJwtAuthorityStateResponse);

    /// PrepareJWTAuthority prepares a new JWT authority for use by
    /// generating a new key and injecting it into the bundle. This action
    /// will propagate the new public key cluster-wide.
    fn prepare_jwt_authority("PrepareJWTAuthority")(PrepareJwtAuthorityRequest) -> (PrepareJwtAuthorityResponse);

    /// ActivateJWTAuthority activates a prepared JWT authority for use,
    /// which will cause it to be used for all JWT signing operations
    /// serviced by this server going forward. If a new JWT authority has
    /// not already been prepared, a FailedPrecondition error will be returned.
    fn activate_jwt_authority("ActivateJWTAuthority")(ActivateJwtAuthorityRequest) -> (ActivateJwtAuthorityResponse);

    /// TaintJWTAuthority marks the previously active JWT authority as
    /// being tainted. SPIRE Agents observing an authority to be tainted
    /// will perform proactive rotations of any key material related to
    /// the tainted authority. The result of this action will be observed
    /// cluster-wide.
    /// It can receive the Authority ID of an old JWT authority.
    //
    /// If a previously active JWT authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn taint_jwt_authority("TaintJWTAuthority")(TaintJwtAuthorityRequest) -> (TaintJwtAuthorityResponse);

    /// RevokeJWTAuthority revokes the previously active JWT authority by
    /// removing it from the bundle and propagating this update throughout
    /// the cluster.
    /// It can receive the Authority ID of an old JWT authority.
    //
    /// If a previously active JWT authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn revoke_jwt_authority("RevokeJWTAuthority")(RevokeJwtAuthorityRequest) -> (RevokeJwtAuthorityResponse);

    /// GetX509AuthorityState returns the state of all locally configured
    /// X.509 authorities.
    fn get_x509_authority_state("GetX509AuthorityState")(GetX509AuthorityStateRequest) -> (GetX509AuthorityStateResponse);

    /// PrepareX509Authority prepares a new X.509 authority for use by
    /// generating a new key and injecting the resulting CA certificate into
    /// the bundle. This action will  propagate the new CA cluster-wide.
    fn prepare_x509_authority("PrepareX509Authority")(PrepareX509AuthorityRequest) -> (PrepareX509AuthorityResponse);

    /// ActivateX509Authority activates a prepared X.509 authority for use,
    /// which will cause it to be used for all X.509 signing operations
    /// serviced by this server going forward. If a new X.509 authority has
    /// not already been prepared, a FailedPrecondition error will be returned.
    fn activate_x509_authority("ActivateX509Authority")(ActivateX509AuthorityRequest) -> (ActivateX509AuthorityResponse);

    /// TaintX509Authority marks the previously active X.509 authority as
    /// being tainted. SPIRE Agents observing an authority to be tainted
    /// will perform proactive rotations of any key material related to
    /// the tainted authority. The result of this action will be observed
    /// cluster-wide.
    /// The X.509 authority to taint is identified using the provided X.509 Subject Key
    //
    /// If an upstream authority is configured then local authorities cannot be tainted,
    /// and a FailedPrecondition error will be returned.
    //
    /// If a previously active X.509 authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn taint_x509_authority("TaintX509Authority")(TaintX509AuthorityRequest) -> (TaintX509AuthorityResponse);

    /// TaintX509UpstreamAuthority marks the provided upstream authority as
    /// being tainted. SPIRE Agents observing a tainted authority
    /// will perform proactive rotations of any key material related to
    /// the tainted authority. The result of this action will be observed
    /// cluster-wide.
    /// It is important to change to a new active upstream authority before tainting the old one,
    /// since tainting will force the rotation of any bundle that is using
    /// the old upstream authority.
    /// The X.509 authority to taint is identified using the provided X.509 Subject Key
    /// Identifier (or SKID) of the old X.509 authority.
    //
    /// If an X.509 upstream authority is not configured, or the identified upstream
    /// X.509 authority is active, a FailedPrecondition error will be returned.
    fn taint_x509_upstream_authority("TaintX509UpstreamAuthority")(TaintX509UpstreamAuthorityRequest) -> (TaintX509UpstreamAuthorityResponse);

    /// RevokeX509Authority revokes the previously active X.509 authority by
    /// removing it from the bundle and propagating this update throughout
    /// the cluster.
    /// It can receive the public key of an old X.509 authority.
    //
    /// If a previously active X.509 authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn revoke_x509_authority("RevokeX509Authority")(RevokeX509AuthorityRequest) -> (RevokeX509AuthorityResponse);

    /// RevokeX509UpstreamAuthority revokes the previously active X.509 upstream authority by
    /// removing it from the bundle and propagating this update throughout
    /// the cluster.
    /// The X.509 authority to revoke is identified using the provided subject key ID of
    /// the authority's CA certificate.
    //
    /// If a previously active X.509 upstream authority does not exist, a FailedPrecondition
    /// error will be returned.
    fn revoke_x509_upstream_authority("RevokeX509UpstreamAuthority")(RevokeX509UpstreamAuthorityRequest) -> (RevokeX509UpstreamAuthorityResponse);

    /// GetWITAuthorityState returns the state of all locally configured
    /// WIT authorities.
    fn get_wit_authority_state("GetWITAuthorityState")(GetWitAuthorityStateRequest) -> (GetWitAuthorityStateResponse);

    /// PrepareWITAuthority prepares a new WIT authority for use by
    /// generating a new key and injecting it into the bundle. This action
    /// will propagate the new public key cluster-wide.
    fn prepare_wit_authority("PrepareWITAuthority")(PrepareWitAuthorityRequest) -> (PrepareWitAuthorityResponse);

    /// ActivateWITAuthority activates a prepared WIT authority for use,
    /// which will cause it to be used for all WIT signing operations
    /// serviced by this server going forward. If a new WIT authority has
    /// not already been prepared, a FailedPrecondition error will be returned.
    fn activate_wit_authority("ActivateWITAuthority")(ActivateWitAuthorityRequest) -> (ActivateWitAuthorityResponse);

    /// TaintWITAuthority marks the previously active WIT authority as
    /// being tainted. SPIRE Agents observing an authority to be tainted
    /// will perform proactive rotations of any key material related to
    /// the tainted authority. The result of this action will be observed
    /// cluster-wide.
    /// The WIT authority to taint is identified using the authority ID of
    /// the old WIT authority.
    //
    /// If a previously active WIT authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn taint_wit_authority("TaintWITAuthority")(TaintWitAuthorityRequest) -> (TaintWitAuthorityResponse);

    /// RevokeWITAuthority revokes the previously active WIT authority by
    /// removing it from the bundle and propagating this update throughout
    /// the cluster.
    /// The WIT authority to revoke is identified using the authority ID of
    /// the old WIT authority.
    //
    /// If a previously active WIT authority does not exist (e.g. if one
    /// has been prepared but not activated yet), a FailedPrecondition
    /// error will be returned.
    fn revoke_wit_authority("RevokeWITAuthority")(RevokeWitAuthorityRequest) -> (RevokeWitAuthorityResponse);
}
