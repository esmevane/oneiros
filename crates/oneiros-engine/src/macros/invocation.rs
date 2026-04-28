//! Render a versioned-protocol value as a populated CLI invocation string.
//!
//! Walks the serde representation of `value` and emits `--{kebab} {value}`
//! pairs for each scalar field. Used by the `versioned!` macro's
//! `to_invocation()` method so hint text, MCP tool examples, and OpenAPI
//! sample-call documentation can derive from the value itself rather than
//! hand-curated strings.

use serde::Serialize;

pub fn render_invocation<T: Serialize>(name: &str, value: &T) -> String {
    let json = match serde_json::to_value(value) {
        Ok(json) => json,
        Err(_) => return name.to_string(),
    };

    let mut parts = vec![name.to_string()];
    if let serde_json::Value::Object(fields) = json {
        for (key, field_value) in fields {
            append_field(&mut parts, &key, &field_value);
        }
    }
    parts.join(" ")
}

fn append_field(parts: &mut Vec<String>, key: &str, value: &serde_json::Value) {
    match value {
        serde_json::Value::Null => {}
        serde_json::Value::Bool(false) => {}
        serde_json::Value::Bool(true) => parts.push(format!("--{key}")),
        serde_json::Value::String(text) if text.is_empty() => {}
        serde_json::Value::String(text) => parts.push(format!("--{key} {}", shell_escape(text))),
        serde_json::Value::Number(number) => parts.push(format!("--{key} {number}")),
        serde_json::Value::Array(items) => {
            for item in items {
                append_field(parts, key, item);
            }
        }
        serde_json::Value::Object(_) => {
            parts.push(format!("--{key} {value}"));
        }
    }
}

fn shell_escape(text: &str) -> String {
    if text
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/' | ':'))
    {
        text.to_string()
    } else {
        let escaped = text.replace('\'', r"'\''");
        format!("'{escaped}'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Sample {
        name: String,
        description: String,
        verbose: bool,
        count: u32,
    }

    #[test]
    fn renders_string_and_number_fields() {
        let sample = Sample {
            name: "alpha".to_string(),
            description: "hello world".to_string(),
            verbose: true,
            count: 7,
        };
        let invocation = render_invocation("create", &sample);
        assert!(invocation.starts_with("create"));
        assert!(invocation.contains("--name alpha"));
        assert!(invocation.contains("--description 'hello world'"));
        assert!(invocation.contains("--verbose"));
        assert!(invocation.contains("--count 7"));
    }

    #[test]
    fn elides_empty_strings_and_false_bools() {
        let sample = Sample {
            name: "alpha".to_string(),
            description: String::new(),
            verbose: false,
            count: 0,
        };
        let invocation = render_invocation("create", &sample);
        assert!(!invocation.contains("--description"));
        assert!(!invocation.contains("--verbose"));
        assert!(invocation.contains("--count 0"));
    }
}
