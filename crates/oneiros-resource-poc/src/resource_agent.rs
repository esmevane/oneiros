use oneiros_db::Projection;
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

impl Agent {
    /// Projections this resource needs to maintain its read model.
    pub fn projections() -> &'static [Projection] {
        crate::projections::AGENT
    }
}
