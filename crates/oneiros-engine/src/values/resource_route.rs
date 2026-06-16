#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub(crate) struct ResourceRoute(String);

impl ResourceRoute {
    pub(crate) fn new(label: impl AsRef<str>) -> Self {
        Self(label.as_ref().into())
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ResourceRoute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ResourceRoute {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for ResourceRoute {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for ResourceRoute {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl schemars::JsonSchema for ResourceRoute {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ResourceRoute")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}
