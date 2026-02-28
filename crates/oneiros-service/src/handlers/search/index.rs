use std::collections::HashSet;

use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct SearchParams {
    pub q: String,
    pub agent: Option<AgentName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResults>, Error> {
    let mut results = ticket.db.search_expressions(&params.q)?;

    if let Some(ref agent_name) = params.agent {
        let agent = ticket
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let mut owned_refs: HashSet<Ref> = HashSet::new();

        owned_refs.insert(Ref::agent(agent.id));

        for id in ticket.db.list_cognition_ids_by_agent(&agent.id)? {
            owned_refs.insert(Ref::cognition(id));
        }
        for id in ticket.db.list_memory_ids_by_agent(&agent.id)? {
            owned_refs.insert(Ref::memory(id));
        }
        for id in ticket.db.list_experience_ids_by_agent(&agent.id)? {
            owned_refs.insert(Ref::experience(id));
        }

        results.retain(|expr| {
            // Shared resources (personas, vocabulary types) always included
            let label = expr.resource_ref.resource().label();
            matches!(
                label,
                "persona" | "texture" | "level" | "sensation" | "nature"
            ) || owned_refs.contains(&expr.resource_ref)
        });
    }

    Ok(Json(SearchResults {
        query: params.q,
        results,
    }))
}
