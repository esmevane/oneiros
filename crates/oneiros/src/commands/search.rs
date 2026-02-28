use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SearchError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SearchOutcomes {
    #[outcome(message("{}", .0))]
    Results(SearchResultsDisplay),
}

#[derive(Clone, serde::Serialize)]
pub struct SearchResultsDisplay(SearchResults);

impl std::fmt::Display for SearchResultsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.results.is_empty() {
            return write!(f, "No results for '{}'", self.0.query);
        }

        writeln!(f, "Search results for '{}':\n", self.0.query)?;

        for result in &self.0.results {
            let kind = result.kind.as_str();
            let content = result.content.as_str();
            let truncated = if content.len() > 80 {
                let end = content.floor_char_boundary(80);
                format!("{}...", &content[..end])
            } else {
                content.to_string()
            };
            let ref_token = RefToken::new(result.resource_ref.clone());

            writeln!(f, "  [{kind}] {truncated}")?;
            writeln!(f, "    {ref_token}\n")?;
        }

        Ok(())
    }
}

/// Search the cognitive stream using full-text search.
#[derive(Clone, Args)]
pub struct SearchOp {
    /// The search query (supports FTS5 syntax: AND, OR, NOT, prefix*).
    #[arg(required = true)]
    query: Vec<String>,
}

impl SearchOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<SearchOutcomes>, SearchError> {
        let mut outcomes = Outcomes::new();

        let query = self.query.join(" ");
        let client = Client::new(context.socket_path());
        let results = client.search(&context.ticket_token()?, &query).await?;

        outcomes.emit(SearchOutcomes::Results(SearchResultsDisplay(results)));

        Ok(outcomes)
    }
}
