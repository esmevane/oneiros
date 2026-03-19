use clap::Args;

use crate::*;

pub struct PressureCli;

#[derive(Debug, Args)]
pub struct PressureCommands {
    pub name: String,
}

impl PressureCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: PressureCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = PressureService::get(ctx, &cmd.name)?.into();
        Ok(result)
    }
}
