use oneiros_model::Experience;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowExperienceOutcomes {
    #[outcome(
        message("Experience details: {0:?}"),
        prompt(
            "Does this connect to anything new? Add with `oneiros experience ref add <id> <record_id> <record_kind>`."
        )
    )]
    ExperienceDetails(Experience),
}
