use super::ArtifactFile;

const HOOKS_JSON: &str = include_str!("../../skill/hooks.json");

pub fn artifact() -> ArtifactFile {
    ArtifactFile::new("hooks/hooks.json", HOOKS_JSON)
}
