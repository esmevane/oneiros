use super::ArtifactFile;

const TEMPLATE: &str = include_str!("../../skill/SKILL.md");

pub fn artifact() -> ArtifactFile {
    ArtifactFile::stamped("skills/oneiros/SKILL.md", TEMPLATE)
}
