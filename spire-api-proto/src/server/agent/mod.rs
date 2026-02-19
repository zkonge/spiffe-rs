mod types;

pub use self::types::*;
use crate::{Agent as AgentType, Empty, JoinToken, macros::define_grpc};

define_grpc! {
    /// Agent management API for SPIRE Server.
    Agent,
    AgentClient,
    AgentServer,
    "spire.api.server.agent.v1.Agent",

    fn count_agents("CountAgents")(CountAgentsRequest) -> (CountAgentsResponse);

    fn list_agents("ListAgents")(ListAgentsRequest) -> (ListAgentsResponse);

    fn get_agent("GetAgent")(GetAgentRequest) -> (AgentType);

    fn delete_agent("DeleteAgent")(DeleteAgentRequest) -> (Empty);

    fn ban_agent("BanAgent")(BanAgentRequest) -> (Empty);

    fn attest_agent("AttestAgent")(stream AttestAgentRequest) -> (stream AttestAgentResponse) as AttestAgentStream;

    fn renew_agent("RenewAgent")(RenewAgentRequest) -> (RenewAgentResponse);

    fn create_join_token("CreateJoinToken")(CreateJoinTokenRequest) -> (JoinToken);

    fn post_status("PostStatus")(PostStatusRequest) -> (PostStatusResponse);
}
