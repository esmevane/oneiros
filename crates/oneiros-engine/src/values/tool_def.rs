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

/// Generate a JSON Schema value for a type, flattened for MCP compatibility.
///
/// Schemars produces schemas with `$defs`/`$ref` indirection, `$schema`, and
/// `title` fields. Claude Code (and likely other MCP clients) expects flat
/// inline schemas with `"type": "object"`. This function generates the schema
/// then normalizes it into that flat form.
pub fn schema_for<T: schemars::JsonSchema>() -> serde_json::Value {
    let settings = schemars::generate::SchemaSettings::draft2020_12();
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<T>();
    let mut value = serde_json::to_value(schema).expect("schema serialization should not fail");
    flatten_schema(&mut value);
    value
}

/// Post-process a JSON Schema value into flat MCP-compatible form.
///
/// 1. Collects `$defs` from the root for ref resolution
/// 2. Recursively inlines any `$ref` pointers
/// 3. Strips `$schema`, `title`, and `$defs` from the root
/// 4. Ensures the root has `"type": "object"` (empty-input tools)
fn flatten_schema(root: &mut serde_json::Value) {
    let serde_json::Value::Object(obj) = root else {
        return;
    };

    // Collect definitions for $ref resolution.
    let defs = obj
        .get("$defs")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    // Inline all $ref pointers recursively.
    inline_refs(root, &defs);

    let serde_json::Value::Object(obj) = root else {
        return;
    };

    // Strip meta-fields that MCP clients don't expect.
    obj.remove("$schema");
    obj.remove("title");
    obj.remove("$defs");

    // Ensure the root is typed as an object — parameterless tools
    // (like serde_json::Value) produce schemas without "type".
    if !obj.contains_key("type") {
        obj.insert(
            "type".to_string(),
            serde_json::Value::String("object".to_string()),
        );
    }
}

/// Recursively resolve `$ref` pointers against a definitions map.
fn inline_refs(value: &mut serde_json::Value, defs: &serde_json::Map<String, serde_json::Value>) {
    let serde_json::Value::Object(obj) = value else {
        return;
    };

    // If this node is a $ref, replace it with the referenced definition.
    if let Some(serde_json::Value::String(ref_path)) = obj.get("$ref") {
        let name = ref_path.strip_prefix("#/$defs/").unwrap_or(ref_path);

        if let Some(definition) = defs.get(name) {
            let mut resolved = definition.clone();
            inline_refs(&mut resolved, defs);
            *value = resolved;
            // Strip title from inlined definitions too.
            if let serde_json::Value::Object(obj) = value {
                obj.remove("title");
            }
            return;
        }
    }

    // Recurse into all object values and arrays.
    for (_, child) in obj.iter_mut() {
        match child {
            serde_json::Value::Object(_) => inline_refs(child, defs),
            serde_json::Value::Array(arr) => {
                for item in arr.iter_mut() {
                    inline_refs(item, defs);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A parameterless tool should produce `{"type": "object"}` —
    /// no $schema, no title, no $defs.
    #[test]
    fn parameterless_schema_is_flat_object() {
        let schema = schema_for::<serde_json::Value>();
        let obj = schema.as_object().expect("schema should be an object");

        assert_eq!(obj.get("type").and_then(|v| v.as_str()), Some("object"));
        assert!(!obj.contains_key("$schema"), "should not contain $schema");
        assert!(!obj.contains_key("title"), "should not contain title");
        assert!(!obj.contains_key("$defs"), "should not contain $defs");
    }

    /// A struct with newtype fields should have properties inlined, no $ref.
    #[test]
    fn struct_with_newtype_fields_is_inlined() {
        use crate::GetPressure;

        let schema = schema_for::<GetPressure>();
        let obj = schema.as_object().expect("schema should be an object");

        assert_eq!(obj.get("type").and_then(|v| v.as_str()), Some("object"));
        assert!(!obj.contains_key("$schema"));
        assert!(!obj.contains_key("title"));
        assert!(!obj.contains_key("$defs"));

        // The "agent" property should be inlined as {"type": "string"}, not a $ref.
        let properties = obj.get("properties").and_then(|v| v.as_object());
        assert!(properties.is_some(), "should have properties");

        let agent_prop = properties.unwrap().get("agent").and_then(|v| v.as_object());
        assert!(agent_prop.is_some(), "should have 'agent' property");
        assert_eq!(
            agent_prop.unwrap().get("type").and_then(|v| v.as_str()),
            Some("string"),
            "agent should be inlined as type: string"
        );
        assert!(
            !agent_prop.unwrap().contains_key("$ref"),
            "agent should not use $ref"
        );
    }
}
