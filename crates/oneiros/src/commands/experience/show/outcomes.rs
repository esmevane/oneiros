use oneiros_model::Experience;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ExperienceDetail(pub Experience);

impl core::fmt::Display for ExperienceDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_detail())
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowExperienceOutcomes {
    #[outcome(
        message("{0}"),
        prompt(
            "Does this connect to anything new? Add with `oneiros experience ref add <id> <record_id> <record_kind>`."
        )
    )]
    ExperienceDetails(ExperienceDetail),
}
