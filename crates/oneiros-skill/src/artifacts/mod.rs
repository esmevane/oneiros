mod commands;
mod plugin;
mod resources;
mod skill_md;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct ArtifactFile {
    pub path: &'static str,
    pub content: String,
}

impl ArtifactFile {
    fn new(path: &'static str, content: impl Into<String>) -> Self {
        Self {
            path,
            content: content.into(),
        }
    }

    fn stamped(path: &'static str, template: &str) -> Self {
        Self::new(path, template.replace("{{VERSION}}", VERSION))
    }
}

/// The AGENTS.md section content for non-Claude agent runtimes.
/// This is handled separately from other artifacts — it gets appended
/// to an existing AGENTS.md or used to create a new one.
pub const AGENTS_MD_SECTION: &str = include_str!("../../skill/agents-md.md");

pub fn all() -> Vec<ArtifactFile> {
    let mut files = Vec::new();

    files.push(skill_md::artifact());
    files.push(plugin::artifact());
    files.extend(commands::artifacts());
    files.extend(resources::artifacts());

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_expected_file_count() {
        let files = all();
        // SKILL.md + plugin.json + commands + resources
        assert!(
            files.len() > 5,
            "Expected more than 5 artifacts, got {}",
            files.len()
        );
    }

    #[test]
    fn skill_md_contains_version() {
        let files = all();
        let skill = files.iter().find(|f| f.path.contains("SKILL.md")).unwrap();
        assert!(
            skill.content.contains(VERSION),
            "SKILL.md should contain version"
        );
    }

    #[test]
    fn plugin_json_is_valid_json() {
        let files = all();
        let plugin = files
            .iter()
            .find(|f| f.path.contains("plugin.json"))
            .unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&plugin.content).expect("plugin.json should be valid JSON");
        assert_eq!(parsed["name"], "oneiros");
    }

    #[test]
    fn no_empty_content() {
        for file in all() {
            assert!(
                !file.content.trim().is_empty(),
                "Artifact {} has empty content",
                file.path
            );
        }
    }

    #[test]
    fn all_paths_are_relative() {
        for file in all() {
            assert!(
                !file.path.starts_with('/'),
                "Artifact path should be relative: {}",
                file.path
            );
        }
    }
}
