use super::ArtifactFile;

const TEMPLATE: &str = include_str!("../../skill/plugin.json");

pub fn artifact() -> ArtifactFile {
    ArtifactFile::stamped(".claude-plugin/plugin.json", TEMPLATE)
}
