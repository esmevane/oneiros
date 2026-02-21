use oneiros_model::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ExperienceList(pub Vec<ExperienceRecord>);

impl core::fmt::Display for ExperienceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|experience| experience.as_table_row(&experience.description, &experience.refs))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListExperiencesOutcomes {
    #[outcome(message("No experiences found."))]
    NoExperiences,

    #[outcome(
        message("{0}"),
        prompt("Which threads are still growing? Extend with `oneiros experience ref add`.")
    )]
    Experiences(ExperienceList),
}
