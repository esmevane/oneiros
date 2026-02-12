use oneiros_model::Agent;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListAgentsOutcomes {
    #[outcome(message("No agents configured."))]
    NoAgents,

    #[outcome(message("Agents: {0:?}"))]
    Agents(Vec<Agent>),
}
