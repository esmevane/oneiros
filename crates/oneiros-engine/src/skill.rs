//! Skill inventory — command documentation and package assets.
//!
//! `SkillInventory` collects command skill documents from every domain
//! and provides the complete skill package for installation.
//!
//! `SkillPackage` represents the installable artifact — everything Claude Code
//! needs to use oneiros as a skill: the SKILL.md, plugin metadata, hooks,
//! agent definitions, resources, and command documentation.
use crate::*;

/// The version of the package, stamped at compile time.
const VERSION: &str = env!("CARGO_PKG_VERSION");

const SKILL_MD: &str = include_str!("../templates/skills/oneiros/SKILL.md");
const PLUGIN_JSON: &str = include_str!("../templates/skills/oneiros/plugin.json");
const HOOKS_JSON: &str = include_str!("../templates/skills/oneiros/hooks.json");
const MARKETPLACE_JSON: &str = include_str!("../templates/skills/oneiros/marketplace.json");
const AGENTS_MD: &str = include_str!("../templates/skills/oneiros/agents-md.md");
const MORNING_PAGES_MD: &str = include_str!("../templates/skills/oneiros-morning-pages/SKILL.md");
const EVENING_PAGES_MD: &str = include_str!("../templates/skills/oneiros-evening-pages/SKILL.md");

/// A file in the skill package — name and content, ready to write.
pub struct SkillAsset {
    /// Relative path within the install target (e.g., "commands/dream.md").
    pub path: &'static str,
    /// File content, already version-stamped if applicable.
    pub content: String,
}

/// The complete skill inventory across all domains.
pub(crate) struct SkillInventory;

impl SkillInventory {
    /// All command skill documents from every domain.
    pub(crate) fn all() -> Vec<Skill> {
        let mut skills = Vec::new();

        skills.extend(ActorRequest::skills());
        skills.extend(AgentRequest::skills());
        skills.extend(BookmarkRequest::skills());
        skills.extend(CognitionRequest::skills());
        skills.extend(ConnectionRequest::skills());
        skills.extend(ContinuityRequest::skills());
        skills.extend(DoctorSkills::all());
        skills.extend(ExperienceRequest::skills());
        skills.extend(FollowRequest::skills());
        skills.extend(LevelRequest::skills());
        skills.extend(McpConfigSkills::all());
        skills.extend(MemoryRequest::skills());
        skills.extend(NatureRequest::skills());
        skills.extend(PeerRequest::skills());
        skills.extend(PersonaRequest::skills());
        skills.extend(PressureRequest::skills());
        skills.extend(ProjectRequest::skills());
        skills.extend(SearchRequest::skills());
        skills.extend(SeedRequest::skills());
        skills.extend(SensationRequest::skills());
        skills.extend(SetupSkills::all());
        skills.extend(SliceRequest::skills());
        skills.extend(StorageRequest::skills());
        skills.push(Skill::new(
            "storage-get",
            include_str!("domains/storage/features/skills/get.md"),
        ));
        skills.extend(HostRequest::skills());
        skills.extend(HostServiceSkills::all());
        skills.extend(LensRequest::skills());
        skills.extend(TenantRequest::skills());
        skills.extend(TextureRequest::skills());
        skills.extend(TicketRequest::skills());
        skills.extend(TrailRequest::skills());
        skills.extend(UrgeRequest::skills());

        skills
    }
}

/// The installable skill package — everything Claude Code needs.
pub struct SkillPackage;

impl SkillPackage {
    /// All files in the package, ready to write to disk.
    ///
    /// Version placeholders (`{{VERSION}}`) are stamped at call time.
    pub fn assets() -> Vec<SkillAsset> {
        let mut assets = vec![
            SkillAsset {
                path: "skills/oneiros/SKILL.md",
                content: Self::stamp(SKILL_MD),
            },
            SkillAsset {
                path: ".claude-plugin/plugin.json",
                content: Self::stamp(PLUGIN_JSON),
            },
            SkillAsset {
                path: ".claude-plugin/marketplace.json",
                content: Self::stamp(MARKETPLACE_JSON),
            },
            SkillAsset {
                path: "hooks/hooks.json",
                content: HOOKS_JSON.to_string(),
            },
            SkillAsset {
                path: "agents-md.md",
                content: AGENTS_MD.to_string(),
            },
            SkillAsset {
                path: "skills/oneiros-morning-pages/SKILL.md",
                content: Self::stamp(MORNING_PAGES_MD),
            },
            SkillAsset {
                path: "skills/oneiros-evening-pages/SKILL.md",
                content: Self::stamp(EVENING_PAGES_MD),
            },
        ];

        // Agent definitions
        for (name, content) in Self::agents() {
            assets.push(SkillAsset {
                path: name,
                content: content.to_string(),
            });
        }

        // Resources
        for (name, content) in Self::resources() {
            assets.push(SkillAsset {
                path: name,
                content: content.to_string(),
            });
        }

        // Command documentation (from the skill inventory)
        for skill in SkillInventory::all() {
            assets.push(SkillAsset {
                path: leak_path(&format!("commands/{}.md", skill.name)),
                content: skill.content.to_string(),
            });
        }

        assets
    }

    /// Agent definition files.
    fn agents() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "agents/activity.scribe.md",
                include_str!("../templates/skills/oneiros/agents/activity.scribe.md"),
            ),
            (
                "agents/oneiroi.process.md",
                include_str!("../templates/skills/oneiros/agents/oneiroi.process.md"),
            ),
        ]
    }

    /// Resource files.
    fn resources() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "skills/oneiros/resources/cognitive-model.md",
                include_str!("../templates/skills/oneiros/resources/cognitive-model.md"),
            ),
            (
                "skills/oneiros/resources/getting-started.md",
                include_str!("../templates/skills/oneiros/resources/getting-started.md"),
            ),
        ]
    }

    /// Replace `{{VERSION}}` with the current package version.
    fn stamp(content: &str) -> String {
        content.replace("{{VERSION}}", VERSION)
    }
}

/// Leak a String into a &'static str for SkillAsset paths.
///
/// This is fine for the skill package — assets are built once per install
/// and the leaked strings live for the program's lifetime.
fn leak_path(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_is_not_empty() {
        let skills = SkillInventory::all();
        assert!(!skills.is_empty(), "skill inventory should not be empty");
    }

    #[test]
    fn all_skills_have_content() {
        for skill in SkillInventory::all() {
            assert!(
                !skill.content.trim().is_empty(),
                "skill '{}' has empty content",
                skill.name
            );
        }
    }

    #[test]
    fn all_skill_names_are_unique() {
        let skills = SkillInventory::all();
        let mut names: Vec<&str> = skills.iter().map(|s| s.name.as_ref()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), skills.len(), "duplicate skill names found");
    }

    #[test]
    fn level_skills_are_present() {
        let skills = SkillInventory::all();
        let level_skills: Vec<_> = skills
            .iter()
            .filter(|s| {
                matches!(
                    s.name.as_ref(),
                    "set-level" | "get-level" | "list-levels" | "remove-level"
                )
            })
            .collect();
        assert_eq!(
            level_skills.len(),
            4,
            "expected 4 level skills (set, get, list, remove), got {}",
            level_skills.len()
        );
    }

    #[test]
    fn continuity_skills_are_present() {
        let skills = SkillInventory::all();
        let continuity_skills: Vec<_> = skills
            .iter()
            .filter(|s| {
                matches!(
                    s.name.as_ref(),
                    "wake-agent"
                        | "dream-agent"
                        | "introspect-agent"
                        | "reflect-agent"
                        | "sense-content"
                        | "sleep-agent"
                        | "guidebook-agent"
                        | "emerge-agent"
                        | "recede-agent"
                        | "status-agent"
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
        let skills = SkillInventory::all();
        let cases: &[(&str, &[&str])] = &[
            (
                "texture",
                &[
                    "set-texture",
                    "get-texture",
                    "list-textures",
                    "remove-texture",
                ],
            ),
            (
                "sensation",
                &[
                    "set-sensation",
                    "get-sensation",
                    "list-sensations",
                    "remove-sensation",
                ],
            ),
            (
                "nature",
                &["set-nature", "get-nature", "list-natures", "remove-nature"],
            ),
            (
                "persona",
                &[
                    "set-persona",
                    "get-persona",
                    "list-personas",
                    "remove-persona",
                ],
            ),
            (
                "urge",
                &["set-urge", "get-urge", "list-urges", "remove-urge"],
            ),
        ];

        for (domain, new_names) in cases {
            // Check both naming conventions: new (verb-domain) and
            // legacy (domain-verb) until all vocabulary domains are converted
            // to the noteworthy annotation pattern.
            let legacy_prefix = format!("{domain}-");
            let domain_skills: Vec<_> = skills
                .iter()
                .filter(|s| {
                    new_names.contains(&s.name.as_ref()) || s.name.starts_with(&legacy_prefix)
                })
                .collect();
            assert_eq!(
                domain_skills.len(),
                4,
                "expected 4 skills for domain '{domain}', got {}",
                domain_skills.len()
            );
        }
    }

    #[test]
    fn package_assets_include_skill_md() {
        let assets = SkillPackage::assets();
        let skill_md = assets.iter().find(|a| a.path == "skills/oneiros/SKILL.md");
        assert!(skill_md.is_some(), "package should include SKILL.md");
        assert!(
            !skill_md.unwrap().content.contains("{{VERSION}}"),
            "SKILL.md should have version stamped"
        );
    }

    #[test]
    fn package_assets_include_commands() {
        let assets = SkillPackage::assets();
        let commands: Vec<_> = assets
            .iter()
            .filter(|a| a.path.starts_with("commands/"))
            .collect();
        assert!(
            !commands.is_empty(),
            "package should include command documentation"
        );
        // Should match the skill inventory count
        assert_eq!(
            commands.len(),
            SkillInventory::all().len(),
            "command count should match skill inventory"
        );
    }

    #[test]
    fn package_version_is_stamped() {
        let assets = SkillPackage::assets();
        let plugin = assets
            .iter()
            .find(|a| a.path == ".claude-plugin/plugin.json")
            .expect("package should include plugin.json");
        assert!(
            plugin.content.contains(VERSION),
            "plugin.json should contain the current version"
        );
    }
}
