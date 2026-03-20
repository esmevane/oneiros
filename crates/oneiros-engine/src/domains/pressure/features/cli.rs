use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct PressureCommands {
    pub name: String,
}

impl PressureCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, PressureError> {
        let result = PressureService::get(context, &AgentName::new(&self.name))?.into();
        Ok(result)
    }
}
