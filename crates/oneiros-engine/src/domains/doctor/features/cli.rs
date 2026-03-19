use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub fn execute(ctx: &SystemContext) -> Result<String, Box<dyn std::error::Error>> {
        let results = DoctorService::check(ctx);
        Ok(serde_json::to_string_pretty(&results)?)
    }
}
