/// A tool definition — name, description, and JSON schema for input parameters.
///
/// Each domain declares its tools as static slices of ToolDef.
/// The MCP collector gathers them into the tool catalog.
pub struct ToolDef {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: fn() -> serde_json::Value,
}

/// Generate a JSON Schema value for a type.
///
/// Uses schemars draft2020-12 settings to match the MCP specification.
pub fn schema_for<T: schemars::JsonSchema>() -> serde_json::Value {
    let settings = schemars::generate::SchemaSettings::draft2020_12();
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<T>();
    serde_json::to_value(schema).expect("schema serialization should not fail")
}
