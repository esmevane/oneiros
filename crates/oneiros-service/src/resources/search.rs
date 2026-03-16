use oneiros_model::*;

use crate::*;

pub struct SearchStore;

impl Dispatch<SearchRequests> for SearchStore {
    type Response = SearchResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, SearchRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            SearchRequests::Search(request) => {
                let mut results = db.search_expressions(&request.query)?;

                if let Some(agent_name) = &request.agent {
                    let agent = db
                        .get_agent(agent_name)?
                        .ok_or(NotFound::Agent(agent_name.clone()))?;

                    let mut owned_refs: std::collections::HashSet<Ref> =
                        std::collections::HashSet::new();

                    owned_refs.insert(Ref::agent(agent.id));

                    for id in db.list_cognition_ids_by_agent(&agent.id)? {
                        owned_refs.insert(Ref::cognition(id));
                    }
                    for id in db.list_memory_ids_by_agent(&agent.id)? {
                        owned_refs.insert(Ref::memory(id));
                    }
                    for id in db.list_experience_ids_by_agent(&agent.id)? {
                        owned_refs.insert(Ref::experience(id));
                    }

                    results.retain(|expr| {
                        let label = expr.resource_ref.resource().label();
                        matches!(
                            label,
                            "persona" | "texture" | "level" | "sensation" | "nature"
                        ) || owned_refs.contains(&expr.resource_ref)
                    });
                }

                Ok(SearchResponses::SearchComplete(SearchResults {
                    query: request.query,
                    results,
                }))
            }
        }
    }
}
