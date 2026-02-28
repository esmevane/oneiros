use clap::Args;
use oneiros_client::Client;
use oneiros_model::{ExperienceId, ExperienceRef, Label, RefToken};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceCreatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {}", .0.ref_token), prompt("{}", .0.gauge))]
    ExperienceCreated(ExperienceCreatedResult),
}

#[derive(Clone, Args)]
pub struct CreateExperience {
    /// The agent who is creating this experience.
    agent: AgentName,

    /// The sensation of experience being created.
    sensation: SensationName,

    /// A description of the experience.
    description: Description,

    /// References in the format: ref-string or ref-string:role
    #[arg(long = "ref")]
    refs: Vec<String>,
}

impl CreateExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let refs = resolve_refs(&self.refs)?;

        let experience = client
            .create_experience(
                &token,
                CreateExperienceRequest {
                    agent: self.agent.clone(),
                    sensation: self.sensation.clone(),
                    description: self.description.clone(),
                    refs,
                },
            )
            .await?;

        let all = client
            .list_experiences(&token, Some(&self.agent), None)
            .await?;
        let gauge = crate::gauge::experience_gauge(&self.agent, &all);

        let ref_token = experience.ref_token();

        outcomes.emit(CreateExperienceOutcomes::ExperienceCreated(
            ExperienceCreatedResult {
                id: experience.id,
                ref_token,
                gauge,
            },
        ));

        Ok(outcomes)
    }
}

/// Parse ref strings in the format: `<ref-token>` or `<ref-token>:<role>`.
///
/// RefTokens may have an optional `ref:` prefix (e.g. `ref:AAAQAZyg...`).
/// The role delimiter is a colon after the ref-token portion. Since ref-tokens
/// use base64url (no colons), any colon after the `ref:` prefix is the role delimiter.
fn resolve_refs(ref_strings: &[String]) -> Result<Vec<ExperienceRef>, ExperienceCommandError> {
    let mut refs = Vec::new();

    for ref_str in ref_strings {
        // Split on the last colon to separate an optional role suffix.
        // RefTokens may start with "ref:" — try parsing the full string first.
        let (token_str, role) = match ref_str.rsplit_once(':') {
            Some((left, right)) => {
                // Try parsing the full string as a RefToken first — if it works,
                // there's no role suffix.
                if ref_str.parse::<RefToken>().is_ok() {
                    (ref_str.as_str(), None)
                } else {
                    (left, Some(Label::new(right)))
                }
            }
            None => (ref_str.as_str(), None),
        };

        let token: RefToken = token_str
            .parse()
            .map_err(|e| ExperienceCommandError::InvalidRefFormat(format!("{e}")))?;

        refs.push(ExperienceRef::new(token.into_inner(), role));
    }

    Ok(refs)
}
