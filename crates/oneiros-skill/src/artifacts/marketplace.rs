use super::ArtifactFile;

const TEMPLATE: &str = include_str!("../../skill/marketplace.json");

pub fn artifact() -> ArtifactFile {
    ArtifactFile::stamped(".claude-plugin/marketplace.json", TEMPLATE)
}
