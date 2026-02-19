use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetJwtAuthorityStateRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct GetJwtAuthorityStateResponse {
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    #[prost(message, optional, tag = "3")]
    pub old: Option<AuthorityState>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PrepareJwtAuthorityRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct PrepareJwtAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub prepared_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct ActivateJwtAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ActivateJwtAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub activated_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintJwtAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct TaintJwtAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub tainted_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeJwtAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct RevokeJwtAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub revoked_authority: Option<AuthorityState>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetX509AuthorityStateRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct GetX509AuthorityStateResponse {
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    #[prost(message, optional, tag = "3")]
    pub old: Option<AuthorityState>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PrepareX509AuthorityRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct PrepareX509AuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub prepared_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct ActivateX509AuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ActivateX509AuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub activated_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintX509AuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct TaintX509AuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub tainted_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintX509UpstreamAuthorityRequest {
    #[prost(string, tag = "1")]
    pub subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintX509UpstreamAuthorityResponse {
    #[prost(string, tag = "1")]
    pub upstream_authority_subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509UpstreamAuthorityRequest {
    #[prost(string, tag = "1")]
    pub subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509UpstreamAuthorityResponse {
    #[prost(string, tag = "1")]
    pub upstream_authority_subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509AuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct RevokeX509AuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub revoked_authority: Option<AuthorityState>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetWitAuthorityStateRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct GetWitAuthorityStateResponse {
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    #[prost(message, optional, tag = "3")]
    pub old: Option<AuthorityState>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PrepareWitAuthorityRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct PrepareWitAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub prepared_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct ActivateWitAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ActivateWitAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub activated_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintWitAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct TaintWitAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub tainted_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeWitAuthorityRequest {
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct RevokeWitAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub revoked_authority: Option<AuthorityState>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct AuthorityState {
    #[prost(string, tag = "1")]
    pub authority_id: String,

    #[prost(int64, tag = "2")]
    pub expires_at: i64,

    #[prost(string, tag = "3")]
    pub upstream_authority_subject_key_id: String,
}
