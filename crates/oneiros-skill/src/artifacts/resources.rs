use super::ArtifactFile;

macro_rules! resource {
    ($file:literal) => {
        ArtifactFile::new(
            concat!("skills/oneiros/resources/", $file),
            include_str!(concat!("../../skill/resources/", $file)),
        )
    };
}

pub fn artifacts() -> Vec<ArtifactFile> {
    vec![
        resource!("getting-started.md"),
        resource!("cognitive-model.md"),
    ]
}
