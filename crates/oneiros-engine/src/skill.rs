//! Skill inventory — command documentation and package assets.
//!
//! `SkillInventory` collects command skill documents from every domain
//! and provides the complete skill package for installation.
//!
//! `SkillPackage` represents the installable artifact — everything Claude Code
//! needs to use oneiros as a skill: the SKILL.md, plugin metadata, hooks,
//! agent definitions, resources, and command documentation.

use std::path::Path;

use crate::*;

/// The version of the package, stamped at compile time.
const VERSION: &str = env!("CARGO_PKG_VERSION");

// ── Package assets (embedded at compile time) ────────────────────

const SKILL_MD: &str = include_str!("../templates/skill/SKILL.md");
const PLUGIN_JSON: &str = include_str!("../templates/skill/plugin.json");
const HOOKS_JSON: &str = include_str!("../templates/skill/hooks.json");
const MARKETPLACE_JSON: &str = include_str!("../templates/skill/marketplace.json");
const AGENTS_MD: &str = include_str!("../templates/skill/agents-md.md");

/// A file in the skill package — name and content, ready to write.
pub struct SkillAsset {
    /// Relative path within the install target (e.g., "commands/dream.md").
    pub path: &'static str,
    /// File content, already version-stamped if applicable.
    pub content: String,
}

// ── SkillInventory ───────────────────────────────────────────────

/// The complete skill inventory across all domains.
pub struct SkillInventory;

impl SkillInventory {
    /// All command skill documents from every domain.
    pub fn all() -> Vec<Skill> {
        let mut skills = Vec::new();

        skills.extend(ActorSkills::all());
        skills.extend(AgentSkills::all());
        skills.extend(BrainSkills::all());
        skills.extend(CognitionSkills::all());
        skills.extend(ConnectionSkills::all());
        skills.extend(ContinuitySkills::all());
        skills.extend(DoctorSkills::all());
        skills.extend(ExperienceSkills::all());
        skills.extend(LevelSkills::all());
        skills.extend(MemorySkills::all());
        skills.extend(NatureSkills::all());
        skills.extend(PersonaSkills::all());
        skills.extend(PressureSkills::all());
        skills.extend(ProjectSkills::all());
        skills.extend(SearchSkills::all());
        skills.extend(SeedSkills::all());
        skills.extend(SensationSkills::all());
        skills.extend(ServiceSkills::all());
        skills.extend(StorageSkills::all());
        skills.extend(SystemSkills::all());
        skills.extend(TenantSkills::all());
        skills.extend(TextureSkills::all());
        skills.extend(TicketSkills::all());
        skills.extend(UrgeSkills::all());

        skills
    }
}

// ── SkillPackage ─────────────────────────────────────────────────

/// The installable skill package — everything Claude Code needs.
pub struct SkillPackage;

impl SkillPackage {
    /// The package version.
    pub fn version() -> &'static str {
        VERSION
    }

    /// All files in the package, ready to write to disk.
    ///
    /// Version placeholders (`{{VERSION}}`) are stamped at call time.
    pub fn assets() -> Vec<SkillAsset> {
        let mut assets = Vec::new();

        // Core metadata (version-stamped)
        assets.push(SkillAsset {
            path: "skills/oneiros/SKILL.md",
            content: Self::stamp(SKILL_MD),
        });
        assets.push(SkillAsset {
            path: ".claude-plugin/plugin.json",
            content: Self::stamp(PLUGIN_JSON),
        });
        assets.push(SkillAsset {
            path: ".claude-plugin/marketplace.json",
            content: Self::stamp(MARKETPLACE_JSON),
        });

        // Hooks (no stamping needed)
        assets.push(SkillAsset {
            path: "hooks/hooks.json",
            content: HOOKS_JSON.to_string(),
        });

        // AGENTS.md template
        assets.push(SkillAsset {
            path: "agents-md.md",
            content: AGENTS_MD.to_string(),
        });

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

    /// Install the complete package to a target directory.
    pub fn install(target: &Path) -> Result<usize, std::io::Error> {
        let assets = Self::assets();
        let count = assets.len();

        for asset in assets {
            let dest = target.join(asset.path);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dest, &asset.content)?;
        }

        Ok(count)
    }

    /// Agent definition files.
    fn agents() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "agents/activity.scribe.md",
                include_str!("../templates/skill/agents/activity.scribe.md"),
            ),
            (
                "agents/oneiroi.process.md",
                include_str!("../templates/skill/agents/oneiroi.process.md"),
            ),
        ]
    }

    /// Resource files.
    fn resources() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "skills/oneiros/resources/cognitive-model.md",
                include_str!("../templates/skill/resources/cognitive-model.md"),
            ),
            (
                "skills/oneiros/resources/getting-started.md",
                include_str!("../templates/skill/resources/getting-started.md"),
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
        let mut names: Vec<&str> = skills.iter().map(|s| s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), skills.len(), "duplicate skill names found");
    }

    #[test]
    fn level_skills_are_present() {
        let skills = SkillInventory::all();
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
        let skills = SkillInventory::all();
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
        let skills = SkillInventory::all();
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

    #[test]
    fn package_install_writes_files() {
        let temp = tempfile::TempDir::new().unwrap();
        let count = SkillPackage::install(temp.path()).unwrap();
        assert!(count > 0, "should install at least one file");

        // Verify a few key files exist
        assert!(temp.path().join("skills/oneiros/SKILL.md").exists());
        assert!(temp.path().join(".claude-plugin/plugin.json").exists());
        assert!(temp.path().join("hooks/hooks.json").exists());
        assert!(temp.path().join("commands/dream.md").exists());
    }
}
