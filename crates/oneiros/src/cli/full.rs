use clap::Parser;

use crate::cli::Cli;

#[derive(Clone, Parser)]
#[command(version)]
pub(crate) struct Full {
    #[command(flatten)]
    pub(crate) project: super::ProjectConfig,
    #[command(flatten)]
    pub(crate) log: super::LogConfig,
    #[clap(long, short = 'P', value_parser)]
    pub(crate) project_dir: Option<std::path::PathBuf>,
    #[command(subcommand)]
    pub(crate) command: super::Command,
}

impl Full {
    pub(crate) async fn run(&self) {
        match &self.command {
            super::Command::Doctor(command) => {
                command.run(self.context()).await;
            }
        }
        todo!()
    }
}

impl Cli for Full {
    fn project_name(&self) -> &str {
        &self.project.name
    }

    fn project_dir(&self) -> std::path::PathBuf {
        self.project_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
    }
}
