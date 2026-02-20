use prost::Message;

use crate::{Agent as AgentType, AgentMask, SpiffeId, X509Svid};

pub mod count_agents_request {
    use prost::Message;

    use crate::SelectorMatch;

    #[derive(Clone, PartialEq, Message)]
    pub struct Filter {
        /// Filters agents to those matching the attestation type.
        #[prost(string, tag = "1")]
        pub by_attestation_type: String,

        /// Filters agents to those satisfying the selector match.
        #[prost(message, optional, tag = "2")]
        pub by_selector_match: Option<SelectorMatch>,

        /// Filters agents to those that are banned.
        #[prost(bool, optional, tag = "3")]
        pub by_banned: Option<bool>,

        /// Filters agents that can re-attest.
        #[prost(bool, optional, tag = "4")]
        pub by_can_reattest: Option<bool>,

        /// Filters agents by those expires before.
        #[prost(string, tag = "5")]
        pub by_expires_before: String,
    }
}

/// Request to count agents matching an optional filter.
#[derive(Clone, PartialEq, Message)]
pub struct CountAgentsRequest {
    /// Filters the agents returned by the list operation.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<count_agents_request::Filter>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct CountAgentsResponse {
    #[prost(int32, tag = "1")]
    pub count: i32,
}

pub mod list_agents_request {
    use prost::Message;

    use crate::SelectorMatch;

    #[derive(Clone, PartialEq, Message)]
    pub struct Filter {
        /// Filters agents to those matching the attestation type.
        #[prost(string, tag = "1")]
        pub by_attestation_type: String,

        /// Filters agents to those satisfying the selector match.
        #[prost(message, optional, tag = "2")]
        pub by_selector_match: Option<SelectorMatch>,

        /// Filters agents to those that are banned.
        #[prost(bool, optional, tag = "3")]
        pub by_banned: Option<bool>,

        /// Filters agents that can re-attest.
        #[prost(bool, optional, tag = "4")]
        pub by_can_reattest: Option<bool>,

        /// Filters agents by those expires before.
        #[prost(string, tag = "5")]
        pub by_expires_before: String,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct ListAgentsRequest {
    /// Filters the agents returned by the list operation.
    #[prost(message, optional, tag = "1")]
    pub filter: Option<list_agents_request::Filter>,

    /// An output mask indicating which agent fields are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<AgentMask>,

    /// The maximum number of results to return. The server may further
    /// constrain this value, or if zero, choose its own.
    #[prost(int32, tag = "3")]
    pub page_size: i32,

    /// The next_page_token value returned from a previous request, if any.
    #[prost(string, tag = "4")]
    pub page_token: String,
}

/// Response page for list-agents operation.
#[derive(Clone, PartialEq, Message)]
pub struct ListAgentsResponse {
    /// The agents.
    #[prost(message, repeated, tag = "1")]
    pub agents: Vec<AgentType>,

    /// The page token for the next request. Empty if there are no more results.
    /// This field should be checked by clients even when a page_size was not
    /// requested, since the server may choose its own (see page_size).
    #[prost(string, tag = "2")]
    pub next_page_token: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct GetAgentRequest {
    /// Required. The SPIFFE ID of the agent.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,

    /// An output mask indicating which agent fields are set in the response.
    #[prost(message, optional, tag = "2")]
    pub output_mask: Option<AgentMask>,
}

#[derive(Clone, PartialEq, Message)]
pub struct DeleteAgentRequest {
    /// Required. The SPIFFE ID of the agent.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BanAgentRequest {
    /// Required. The SPIFFE ID of the agent.
    #[prost(message, optional, tag = "1")]
    pub id: Option<SpiffeId>,
}

pub mod attest_agent_request {
    use prost::{Message, Oneof, bytes::Bytes};

    use super::AgentX509SvidParams;
    use crate::AttestationData;

    #[derive(Clone, PartialEq, Message)]
    pub struct Params {
        /// Node attestation data.
        #[prost(message, optional, tag = "1")]
        pub data: Option<AttestationData>,

        /// Requested X509-SVID parameters.
        #[prost(message, optional, tag = "2")]
        pub params: Option<AgentX509SvidParams>,
    }

    /// Request-side step variants for the attestation stream.
    #[derive(Clone, PartialEq, Oneof)]
    pub enum Step {
        /// Attestation parameters. These are only sent in the initial request.
        #[prost(message, tag = "1")]
        Params(Params),

        /// The response to a challenge issued by the attestor. Only sent in
        /// response to a challenge received by the issuer.
        #[prost(bytes, tag = "2")]
        ChallengeResponse(Bytes),
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentRequest {
    /// Required. The data for the step in the attestation flow.
    #[prost(oneof = "attest_agent_request::Step", tags = "1, 2")]
    pub step: Option<attest_agent_request::Step>,
}

pub mod attest_agent_response {
    use prost::{Message, Oneof, bytes::Bytes};

    use crate::X509Svid;

    #[derive(Clone, PartialEq, Message)]
    pub struct Result {
        /// The agent X509-SVID.
        #[prost(message, optional, tag = "1")]
        pub svid: Option<X509Svid>,

        /// Whether or not the attested agent can reattest to renew its X509-SVID
        #[prost(bool, tag = "2")]
        pub reattestable: bool,
    }

    /// Response-side step variants for the attestation stream.
    #[derive(Clone, PartialEq, Oneof)]
    pub enum Step {
        /// Attestation results. If set, attestation has completed.
        #[prost(message, tag = "1")]
        Result(Result),

        /// A challenge issued by the attestor. If set, the caller is expected
        /// to send another request on the stream with the challenge response.
        #[prost(bytes, tag = "2")]
        Challenge(Bytes),
    }
}

/// One step returned by the server during bidirectional agent attestation.
#[derive(Clone, PartialEq, Message)]
pub struct AttestAgentResponse {
    #[prost(oneof = "attest_agent_response::Step", tags = "1, 2")]
    pub step: Option<attest_agent_response::Step>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct AgentX509SvidParams {
    /// Required. The ASN.1 DER encoded Certificate Signing Request (CSR). The
    /// CSR is only used to convey the public key; other fields in the CSR are
    /// ignored. The agent X509-SVID attributes are determined by the server.
    #[prost(bytes = "vec", tag = "1")]
    pub csr: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct RenewAgentRequest {
    /// Required. Parameters for the X509-SVID.
    #[prost(message, optional, tag = "1")]
    pub params: Option<AgentX509SvidParams>,
}

#[derive(Clone, PartialEq, Message)]
pub struct RenewAgentResponse {
    /// The renewed X509-SVID
    #[prost(message, optional, tag = "1")]
    pub svid: Option<X509Svid>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CreateJoinTokenRequest {
    /// Required. How long until the token expires (in seconds).
    #[prost(int32, tag = "1")]
    pub ttl: i32,

    /// An optional token value to use for the token. Must be unique. If unset,
    /// the server will generate a value.
    #[prost(string, tag = "2")]
    pub token: String,

    /// An optional SPIFFE ID to assign to the agent beyond that given by
    /// join token attestation. If set, this results in an entry being created
    /// that maps the attestation assigned agent ID to this ID.
    #[prost(message, optional, tag = "3")]
    pub agent_id: Option<SpiffeId>,
}

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct PostStatusRequest {
    /// Required. Serial number of the bundle currently being served by the agent
    #[prost(uint64, tag = "1")]
    pub current_bundle_serial: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Message)]
pub struct PostStatusResponse {}
