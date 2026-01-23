use clap::Parser;

#[derive(Parser, Default)]
#[command(ignore_errors = true)]
pub(crate) struct Preflight {
    #[command(flatten)]
    pub(crate) project: super::ProjectConfig,
    #[command(flatten)]
    pub(crate) log: super::LogConfig,
}

impl Preflight {
    pub(crate) fn preflight_parse() -> Self {
        Self::try_parse().unwrap_or_default()
    }
}

impl super::Cli for Preflight {
    fn project_name(&self) -> &str {
        &self.project.name
    }

    fn project_dir(&self) -> std::path::PathBuf {
        std::env::current_dir().unwrap_or_default()
    }
}
