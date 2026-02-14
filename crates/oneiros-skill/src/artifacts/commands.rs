use super::ArtifactFile;

macro_rules! command {
    ($file:literal) => {
        ArtifactFile::new(
            concat!("skills/oneiros/commands/", $file),
            include_str!(concat!("../../skill/commands/", $file)),
        )
    };
}

pub fn artifacts() -> Vec<ArtifactFile> {
    vec![
        command!("dream.md"),
        command!("introspect.md"),
        command!("reflect.md"),
        command!("cognition-add.md"),
        command!("cognition-list.md"),
        command!("cognition-show.md"),
        command!("memory-add.md"),
        command!("memory-list.md"),
        command!("memory-show.md"),
        command!("persona-set.md"),
        command!("persona-list.md"),
        command!("persona-show.md"),
        command!("persona-remove.md"),
        command!("agent-create.md"),
        command!("agent-list.md"),
        command!("agent-show.md"),
        command!("agent-update.md"),
        command!("agent-remove.md"),
        command!("texture-set.md"),
        command!("texture-list.md"),
        command!("texture-show.md"),
        command!("texture-remove.md"),
        command!("level-set.md"),
        command!("level-list.md"),
        command!("level-show.md"),
        command!("level-remove.md"),
        command!("storage-set.md"),
        command!("storage-get.md"),
        command!("storage-list.md"),
        command!("storage-show.md"),
        command!("storage-remove.md"),
        command!("system-init.md"),
        command!("project-init.md"),
        command!("service-run.md"),
        command!("service-status.md"),
        command!("service-install.md"),
        command!("service-uninstall.md"),
        command!("service-start.md"),
        command!("service-stop.md"),
        command!("doctor.md"),
        command!("seed.md"),
        command!("skill-install.md"),
    ]
}
