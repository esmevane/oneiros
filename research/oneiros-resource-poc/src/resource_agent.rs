use oneiros_db::Projection;
use oneiros_model::{AgentRequests, AgentResponses};
use oneiros_resource::{Feature, Projections, Resource};

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

impl Feature<Projections> for Agent {
    type Surface = &'static [Projection];

    fn feature(&self) -> Self::Surface {
        crate::projections::AGENT
    }
}

impl Agent {
    pub fn projections() -> &'static [Projection] {
        <Agent as Feature<Projections>>::feature(&Agent)
    }
}
