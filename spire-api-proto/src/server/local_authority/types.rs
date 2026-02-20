use prost::Message;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct GetJwtAuthorityStateRequest {}

#[derive(Clone, PartialEq, Message)]
pub struct GetJwtAuthorityStateResponse {
    /// Authority currently being used for signing operations.
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    /// Authority added on bundle but is not used yet.
    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    /// Authority in that was previously used for signing operations,
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
    /// The authority ID of the local authority JWT authority to activate.
    /// This is the JWT Key ID.
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
    /// The authority ID of the local authority JWT authority to taint.
    /// This is the JWT Key ID.
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
    /// The authority ID of the local authority JWT authority to revoke.
    /// This is the JWT Key ID.
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
    /// Authority currently being used for signing operations.
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    /// Authority added on bundle but is not used yet.
    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    /// Authority in that was previously used for signing operations,
    /// but it is not longer.
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
    /// The authority ID of the local X.509 authority to activate.
    /// This is the X.509 Subject Key Identifier (or SKID) of the
    /// authority's CA certificate, which is calculated by doing a
    /// SHA-1 hash over the ASN.1 encoding of the public key.
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
    /// The authority ID of the local X.509 authority to taint.
    /// This is the X.509 Subject Key Identifier (or SKID) of the
    /// authority's CA certificate, which is calculated by doing a
    /// SHA-1 hash over the ASN.1 encoding of the public key.
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
    /// This is the X.509 Subject Key Identifier (or SKID) of the
    /// authority's CA certificate of the upstream X.509 authority to taint.
    #[prost(string, tag = "1")]
    pub subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct TaintX509UpstreamAuthorityResponse {
    /// The Subject Key Identifier (or SKID) of the upstream authority
    /// tainted.
    #[prost(string, tag = "1")]
    pub upstream_authority_subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509UpstreamAuthorityRequest {
    /// This is the X.509 Subject Key Identifier (or SKID) of the
    /// authority's CA certificate of the upstream X.509 authority to revoke.
    #[prost(string, tag = "1")]
    pub subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509UpstreamAuthorityResponse {
    /// The Subject Key Identifier (or SKID) of the upstream authority
    /// revoked.
    #[prost(string, tag = "1")]
    pub upstream_authority_subject_key_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct RevokeX509AuthorityRequest {
    /// The authority ID of the local X.509 authority to revoke.
    /// This is the X.509 Subject Key Identifier (or SKID) of the
    /// authority's CA certificate, which is calculated by doing a
    /// SHA-1 hash over the ASN.1 encoding of the public key.
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
    /// Authority currently being used for signing operations.
    #[prost(message, optional, tag = "1")]
    pub active: Option<AuthorityState>,

    /// Authority added on bundle but is not used yet.
    #[prost(message, optional, tag = "2")]
    pub prepared: Option<AuthorityState>,

    /// Authority in that was previously used for signing operations,
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
    /// The authority ID of the local authority WIT authority to activate.
    /// This is the WIT Key ID.
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
    /// The authority ID of the local authority WIT authority to taint.
    /// This is the WIT Key ID.
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
    /// The authority ID of the local authority WIT authority to revoke.
    /// This is the WIT Key ID.
    #[prost(string, tag = "1")]
    pub authority_id: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct AuthorityState {
    /// The authority ID.
    #[prost(string, tag = "1")]
    pub authority_id: String,

    /// Expiration timestamp (seconds since Unix epoch).
    #[prost(int64, tag = "2")]
    pub expires_at: i64,

    /// The Subject Key Identifier (or SKID) of the upstream authority,
    /// applicable only for X.509 authorities.
    #[prost(string, tag = "3")]
    pub upstream_authority_subject_key_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct RevokeWitAuthorityResponse {
    #[prost(message, optional, tag = "1")]
    pub revoked_authority: Option<AuthorityState>,
}
