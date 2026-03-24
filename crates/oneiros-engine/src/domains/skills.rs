//! Skill collector — gathers command skill documents from all domains.
//!
//! Each domain that has CLI commands provides a `skills()` function
//! returning its skill documents. This module collects them into a
//! complete inventory for the build-to-dist pipeline.

use crate::Skill;

/// All command skill documents across every domain.
pub fn all() -> Vec<Skill> {
    let mut skills = Vec::new();

    skills.extend(super::actor::skills::skills());
    skills.extend(super::agent::skills::skills());
    skills.extend(super::brain::skills::skills());
    skills.extend(super::cognition::skills::skills());
    skills.extend(super::connection::skills::skills());
    skills.extend(super::continuity::skills::skills());
    skills.extend(super::doctor::skills::skills());
    skills.extend(super::experience::skills::skills());
    skills.extend(super::level::skills::skills());
    skills.extend(super::memory::skills::skills());
    skills.extend(super::nature::skills::skills());
    skills.extend(super::persona::skills::skills());
    skills.extend(super::pressure::skills::skills());
    skills.extend(super::project::skills::skills());
    skills.extend(super::search::skills::skills());
    skills.extend(super::seed::skills::skills());
    skills.extend(super::sensation::skills::skills());
    skills.extend(super::storage::skills::skills());
    skills.extend(super::system::skills::skills());
    skills.extend(super::tenant::skills::skills());
    skills.extend(super::texture::skills::skills());
    skills.extend(super::ticket::skills::skills());
    skills.extend(super::urge::skills::skills());

    skills
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_is_not_empty() {
        let skills = all();
        assert!(!skills.is_empty(), "skill inventory should not be empty");
    }

    #[test]
    fn all_skills_have_content() {
        for skill in all() {
            assert!(
                !skill.content.trim().is_empty(),
                "skill '{}' has empty content",
                skill.name
            );
        }
    }

    #[test]
    fn all_skill_names_are_unique() {
        let skills = all();
        let mut names: Vec<&str> = skills.iter().map(|s| s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), skills.len(), "duplicate skill names found");
    }

    #[test]
    fn level_skills_are_present() {
        let skills = all();
        let level_skills: Vec<_> = skills
            .iter()
            .filter(|s| s.name.starts_with("level-"))
            .collect();
        assert_eq!(
            level_skills.len(),
            4,
            "expected 4 level skills (set, show, list, remove), got {}",
            level_skills.len()
        );
    }

    #[test]
    fn continuity_skills_are_present() {
        let skills = all();
        let continuity_skills: Vec<_> = skills
            .iter()
            .filter(|s| {
                matches!(
                    s.name,
                    "wake"
                        | "dream"
                        | "introspect"
                        | "reflect"
                        | "sense"
                        | "sleep"
                        | "guidebook"
                        | "emerge"
                        | "recede"
                        | "status"
                )
            })
            .collect();
        assert_eq!(
            continuity_skills.len(),
            10,
            "expected 10 continuity skills, got {}",
            continuity_skills.len()
        );
    }

    #[test]
    fn vocabulary_domains_are_complete() {
        let skills = all();
        for domain in &["texture", "sensation", "nature", "persona", "urge"] {
            let domain_skills: Vec<_> = skills
                .iter()
                .filter(|s| s.name.starts_with(domain))
                .collect();
            assert_eq!(
                domain_skills.len(),
                4,
                "expected 4 skills for domain '{domain}', got {}",
                domain_skills.len()
            );
        }
    }
}
