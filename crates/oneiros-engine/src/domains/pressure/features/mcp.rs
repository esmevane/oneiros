use crate::*;

pub struct PressureMcp;

impl PressureMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Pressure.resource_def("All agents' pressure readings")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &PressureRequest,
    ) -> Result<McpResponse, ToolError> {
        pressure_mcp::resource(context, request).await
    }
}

mod pressure_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &PressureRequest,
    ) -> Result<McpResponse, ToolError> {
        let response = match request {
            PressureRequest::GetPressure(get) => PressureService::get(context, get)
                .await
                .map_err(Error::from)?,
            PressureRequest::ListPressures => {
                PressureService::list(context).await.map_err(Error::from)?
            }
        };

        Ok(PressureView::new(response, request)
            .mcp()
            .hint(Hint::inspect(
                ResourcePath::Status.uri(),
                "Agent activity overview",
            )))
    }
}
