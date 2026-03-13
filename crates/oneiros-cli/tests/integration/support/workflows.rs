use oneiros_cli::*;
use oneiros_model::*;

use crate::*;

#[derive(bon::Builder)]
pub(crate) struct Bootstrap {
    #[builder(into, default = "test")]
    tenant: TenantName,
    #[builder(into, default = "test-project")]
    project_name: String,
}

impl Bootstrap {
    pub(crate) async fn run(
        self,
        harness: TestHarness,
    ) -> Result<TestHarness, Box<dyn core::error::Error>> {
        Init::builder()
            .name(self.tenant)
            .yes(true)
            .build()
            .run(harness.context())
            .await?;

        let harness = harness
            .with_project(&self.project_name)
            .with_service()
            .await?;

        InitProject::builder()
            .yes(true)
            .build()
            .run(harness.context())
            .await?;

        Ok(harness)
    }
}
