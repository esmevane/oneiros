/// A tool definition — name + human-readable description.
///
/// Each domain declares its tools as static slices of ToolDef.
/// The MCP collector gathers them into the tool catalog.
pub struct ToolDef {
    pub name: &'static str,
    pub description: &'static str,
}
