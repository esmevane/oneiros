/// A skill command document — markdown describing one CLI operation.
///
/// Each domain produces its skills via `include_str!`, making missing
/// files a compile error. The root collector gathers all domain skills
/// into a complete inventory for the build-to-dist pipeline.
#[derive(Debug, Clone)]
pub struct Skill {
    /// The command name used in the skill file path (e.g. "level-set").
    pub name: &'static str,
    /// The raw markdown content, loaded at compile time.
    pub content: &'static str,
}

impl Skill {
    pub const fn new(name: &'static str, content: &'static str) -> Self {
        Self { name, content }
    }

    /// The dist output path for this skill (e.g. "commands/level-set.md").
    pub fn dist_path(&self) -> String {
        format!("commands/{}.md", self.name)
    }
}
