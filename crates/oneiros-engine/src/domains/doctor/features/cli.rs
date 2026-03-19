use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub fn execute(ctx: &SystemContext) -> Result<Responses, Box<dyn std::error::Error>> {
        let result: Responses = DoctorService::check(ctx).into();
        Ok(result)
    }
}
