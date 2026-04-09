use core::fmt;
use std::marker::PhantomData;

use crate::{Description, ToolName};

/// A type-erased tool definition — name, description, and JSON schema.
///
/// Produced by `Tool<T>::def()`. The MCP collector gathers these into
/// the tool catalog.
pub struct ToolDef {
    pub name: ToolName,
    pub description: Description,
    pub input_schema: serde_json::Value,
}

/// A typed tool — binds a request struct to a name and description.
///
/// The type parameter `T` provides the JSON schema via `schemars::JsonSchema`.
/// The name is derived from the request type's `Display` implementation.
pub struct Tool<T> {
    name: ToolName,
    description: Description,
    _marker: PhantomData<T>,
}

impl<T: schemars::JsonSchema> Tool<T> {
    pub fn new(name: impl fmt::Display, description: impl Into<Description>) -> Self {
        Self {
            name: ToolName::new(name),
            description: description.into(),
            _marker: PhantomData,
        }
    }

    pub fn def(&self) -> ToolDef {
        ToolDef {
            name: self.name.clone(),
            description: self.description.clone(),
            input_schema: schema_for::<T>(),
        }
    }
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
