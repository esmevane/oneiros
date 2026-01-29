use clap::Parser;

use crate::context::Context;

#[derive(Clone, Parser)]
#[command(version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) log: super::LogConfig,
    #[command(subcommand)]
    pub(crate) command: super::Command,
}

impl Cli {
    pub(crate) async fn run(&self) {
        let context = Context::discover();

        match &self.command {
            super::Command::Doctor(command) => {
                command.run(context).await;
            }
        }
    }
}
