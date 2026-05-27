use crate::*;

pub(crate) struct PressureMcp;

impl PressureMcp {
    pub(crate) fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Pressure.resource_def("All agents' pressure readings")]
    }

    #[expect(deprecated)]
    pub(crate) async fn resource(
        &self,
        context: &ProjectLog,
        request: &PressureRequest,
    ) -> Result<McpResponse, ToolError> {
        pressure_mcp::resource(context, request).await
    }
}

mod pressure_mcp {
    use crate::*;

    #[expect(deprecated)]
    pub(crate) async fn resource(
        context: &ProjectLog,
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
