use super::ArtifactFile;

macro_rules! agent {
    ($file:literal) => {
        ArtifactFile::new(
            concat!("agents/", $file),
            include_str!(concat!("../../skill/agents/", $file)),
        )
    };
}

pub fn artifacts() -> Vec<ArtifactFile> {
    vec![
        agent!("oneiroi.process.md"),
        agent!("cognition.scribe.md"),
        agent!("memory.scribe.md"),
        agent!("experience.scribe.md"),
        agent!("storage.scribe.md"),
    ]
}
