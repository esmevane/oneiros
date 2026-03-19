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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result =
            serde_json::to_string_pretty(&PressureService::get(ctx, &cmd.name)?)?;
        Ok(result)
    }
}
