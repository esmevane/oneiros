use oneiros_model::{AgentRequests, AgentResponses};
use oneiros_resource::Resource;

/// The Agent resource declaration.
///
/// Agent is project-scoped — it lives in a brain database, has identity,
/// validation (persona FK), and conflict detection (unique name).
pub struct Agent;

impl Resource for Agent {
    const NAME: &'static str = "agent";

    type Request = AgentRequests;
    type Response = AgentResponses;
}
