use prost::Message;

use crate::{Agent as AgentType, AgentMask, AttestationData, SelectorMatch, SpiffeId, X509Svid};

/// Request to count agents matching an optional filter.
#[derive(Clone, PartialEq, Message)]
pub struct CountAgentsRequest {
    /// Filters applied when counting agents.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<CountAgentsRequestFilter>,
}

/// Filter fields for counting agents.
#[derive(Clone, PartialEq, Message)]
pub struct CountAgentsRequestFilter {
    /// Limits matches to this attestation type.
    #[prost(string, tag = "1")]
    pub by_attestation_type: String,

    /// Limits matches to agents satisfying this selector expression.
    #[prost(message, optional, tag = "2")]
    pub by_selector_match: Option<SelectorMatch>,

    /// If set, filters by banned state.
    #[prost(bool, optional, tag = "3")]
    pub by_banned: Option<bool>,

    /// If set, filters by whether agents can re-attest.
    #[prost(bool, optional, tag = "4")]
    pub by_can_reattest: Option<bool>,

    /// Limits matches to agents expiring before this timestamp string.
    #[prost(string, tag = "5")]
    pub by_expires_before: String,
}

/// Response carrying the number of matched agents.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountAgentsResponse {
    #[prost(int32, tag = "1")]
    pub count: i32,
}

/// Request to list agents with paging and optional filtering.
#[derive(Clone, PartialEq, Message)]
pub struct ListAgentsRequest {
    /// Filters applied when listing agents.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<ListAgentsRequestFilter>,

    /// Field mask controlling which `Agent` fields are returned.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<AgentMask>,

    /// Maximum number of items requested for this page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,

    /// Continuation token from a previous page.
    #[prost(string, tag = "4")]
    pub page_token: String,
}

/// Filter fields for listing agents.
#[derive(Clone, PartialEq, Message)]
pub struct ListAgentsRequestFilter {
    /// Limits matches to this attestation type.
    #[prost(string, tag = "1")]
    pub by_attestation_type: String,

    /// Limits matches to agents satisfying this selector expression.
    #[prost(message, optional, tag = "2")]
    pub by_selector_match: Option<SelectorMatch>,

    /// If set, filters by banned state.
    #[prost(bool, optional, tag = "3")]
    pub by_banned: Option<bool>,

    /// If set, filters by whether agents can re-attest.
    #[prost(bool, optional, tag = "4")]
    pub by_can_reattest: Option<bool>,

    /// Limits matches to agents expiring before this timestamp string.
    #[prost(string, tag = "5")]
    pub by_expires_before: String,
}

/// Response page for list-agents operation.
#[derive(Clone, PartialEq, Message)]
pub struct ListAgentsResponse {
    /// Agents returned for this page.
    #[prost(message, repeated, tag = "1")]
    pub agents: Vec<AgentType>,

    /// Continuation token for the next page, if any.
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

/// Request to fetch one agent by SPIFFE ID.
#[derive(Clone, PartialEq, Message)]
pub struct GetAgentRequest {
    /// Target agent SPIFFE ID.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    /// Field mask controlling which `Agent` fields are returned.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<AgentMask>,
}

/// Request to delete an agent.
#[derive(Clone, PartialEq, Message)]
pub struct DeleteAgentRequest {
    /// Target agent SPIFFE ID.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,
}

/// Request to ban an agent.
#[derive(Clone, PartialEq, Message)]
pub struct BanAgentRequest {
    /// Target agent SPIFFE ID.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,
}

/// One step sent by the caller during bidirectional agent attestation.
#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentRequest {
    /// Attestation step payload.
    #[prost(oneof = "attest_agent_request::Step", tags = "1, 2")]
    pub step: Option<attest_agent_request::Step>,
}

pub mod attest_agent_request {
    use prost::Oneof;

    use super::AttestAgentRequestParams;

    /// Request-side step variants for the attestation stream.
    #[derive(Clone, PartialEq, Oneof)]
    pub enum Step {
        /// Initial attestation parameters.
        #[prost(message, tag = "1")]
        Params(AttestAgentRequestParams),

        /// Response to a challenge issued by the server.
        #[prost(bytes, tag = "2")]
        ChallengeResponse(Vec<u8>),
    }
}

/// Initial attestation parameters for `AttestAgentRequest`.
#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentRequestParams {
    /// Node attestation data.
    #[prost(message, optional, tag = "1")]
    pub data: Option<AttestationData>,

    /// Requested X509-SVID parameters.
    #[prost(message, optional, tag = "2")]
    pub params: Option<AgentX509SvidParams>,
}

/// One step returned by the server during bidirectional agent attestation.
#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentResponse {
    /// Attestation step payload.
    #[prost(oneof = "attest_agent_response::Step", tags = "1, 2")]
    pub step: Option<attest_agent_response::Step>,
}

pub mod attest_agent_response {
    use prost::Oneof;

    use super::AttestAgentResponseResult;

    /// Response-side step variants for the attestation stream.
    #[derive(Clone, PartialEq, Oneof)]
    pub enum Step {
        /// Final attestation result.
        #[prost(message, tag = "1")]
        Result(AttestAgentResponseResult),

        /// Challenge that must be answered by the caller.
        #[prost(bytes, tag = "2")]
        Challenge(Vec<u8>),
    }
}

/// Final successful result of attestation.
#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentResponseResult {
    /// Issued X509-SVID for the attested agent.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<X509Svid>,

    /// Indicates whether this agent is allowed to re-attest.
    #[prost(bool, tag = "2")]
    pub reattestable: bool,
}

/// Request to renew an agent X509-SVID.
#[derive(Clone, PartialEq, Message)]
pub struct RenewAgentRequest {
    /// Parameters for issuing the renewed X509-SVID.
    #[prost(message, optional, tag = "1")]
    pub params: Option<AgentX509SvidParams>,
}

/// Response carrying a renewed X509-SVID.
#[derive(Clone, PartialEq, Message)]
pub struct RenewAgentResponse {
    /// Renewed agent X509-SVID.
    #[prost(message, optional, tag = "1")]
    pub svid: Option<X509Svid>,
}

/// Request to create an agent join token.
#[derive(Clone, PartialEq, Message)]
pub struct CreateJoinTokenRequest {
    /// Token time-to-live in seconds.
    #[prost(int32, tag = "1")]
    pub ttl: i32,

    /// Optional custom token value.
    #[prost(string, tag = "2")]
    pub token: String,

    /// Optional SPIFFE ID assigned to the joining agent.
    #[prost(message, optional, tag = "3")]
    pub agent_id: Option<SpiffeId>,
}

/// Parameters used when issuing an agent X509-SVID.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct AgentX509SvidParams {
    /// ASN.1 DER encoded CSR carrying the public key.
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,
}

/// Request reporting the currently used bundle serial from an agent.
#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct PostStatusRequest {
    /// Serial number of the bundle currently in use by the agent.
    #[prost(uint64, tag = "1")]
    pub current_bundle_serial: u64,
}

/// Empty response for `PostStatus`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PostStatusResponse {}
