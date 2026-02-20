mod types;

pub use self::types::*;
use crate::{Agent as AgentType, JoinToken, macros::define_grpc};

define_grpc! {
    Agent,
    AgentClient,
    AgentServer,
    "spire.api.server.agent.v1.Agent",

    /// Count agents.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn count_agents("CountAgents")(CountAgentsRequest) -> (CountAgentsResponse);

    /// Lists agents.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn list_agents("ListAgents")(ListAgentsRequest) -> (ListAgentsResponse);

    /// Gets an agent.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn get_agent("GetAgent")(GetAgentRequest) -> (AgentType);

    /// Deletes an agent. The agent can come back into the trust domain through
    /// the Issuer AttestAgent RPC.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn delete_agent("DeleteAgent")(DeleteAgentRequest) -> (());

    /// Bans an agent. This evicts the agent and prevents it from rejoining the
    /// trust domain through attestation until the ban is lifted via a call to
    /// DeleteAgent.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn ban_agent("BanAgent")(BanAgentRequest) -> (());

    /// Attests the agent via node attestation, using a bidirectional stream to
    /// faciliate attestation methods that require challenge/response.
    ///
    /// The caller is not authenticated.
    fn attest_agent("AttestAgent")(stream AttestAgentRequest) -> (stream AttestAgentResponse) as AttestAgentStream;


    /// Renews the agent and returns a new X509-SVID. The new SVID is not enabled
    /// on the server side until its first use.
    ///
    /// The caller must present an active agent X509-SVID, i.e. the X509-SVID
    /// returned by the AttestAgent or the most recent RenewAgent call.
    fn renew_agent("RenewAgent")(RenewAgentRequest) -> (RenewAgentResponse);

    /// Creates an agent join token. The token can be used with `join_token`
    /// attestation to join the trust domain.
    ///
    /// The caller must be local or present an admin X509-SVID.
    fn create_join_token("CreateJoinToken")(CreateJoinTokenRequest) -> (JoinToken);

    /// PostStatus post Agent status, informing what's the current
    /// bundle that is being used by the agent.
    ///
    /// The caller must present an active agent X509-SVID, i.e. the X509-SVID
    /// returned by the AttestAgent or the most recent RenewAgent call.
    fn post_status("PostStatus")(PostStatusRequest) -> (PostStatusResponse);
}
