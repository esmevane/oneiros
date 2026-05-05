use crate::*;

pub struct StorageMcp;

impl StorageMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        storage_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        storage_mcp::dispatch(context, mailbox, tool_name, params).await
    }
}

mod storage_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<ListStorage>::new(StorageRequestType::ListStorage, "Browse your archive").def(),
            Tool::<GetStorage>::new(StorageRequestType::GetStorage, "Retrieve a stored artifact")
                .def(),
            Tool::<RemoveStorage>::new(
                StorageRequestType::RemoveStorage,
                "Remove a stored artifact",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: StorageRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = context.scope().map_err(Error::from)?;

        let value = match request_type {
            StorageRequestType::ListStorage => {
                let request: ListStorage = serde_json::from_str(params)
                    .unwrap_or_else(|_| ListStorage::builder_v1().build().into());
                StorageService::list(scope, &request).await
            }
            StorageRequestType::GetStorage => {
                StorageService::show(scope, &serde_json::from_str(params)?).await
            }
            StorageRequestType::RemoveStorage => {
                StorageService::remove(scope, mailbox, &serde_json::from_str(params)?).await
            }
            StorageRequestType::UploadStorage => {
                return Err(ToolError::UnknownTool(tool_name.to_string()));
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
