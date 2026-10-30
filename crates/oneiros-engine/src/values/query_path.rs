/// A request path with percent-encoded query parameters.
///
/// List parameters are often free text (`#317`, `a & b`) or lens source,
/// so every value is form-encoded — a raw `#` would truncate the URL at a
/// fragment, and a raw `&` would split one value into two parameters.
/// Encoding everything spares callers from judging which values are safe.
pub(crate) struct QueryPath {
    path: String,
    params: form_urlencoded::Serializer<'static, String>,
    empty: bool,
}

impl QueryPath {
    pub(crate) fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            params: form_urlencoded::Serializer::new(String::new()),
            empty: true,
        }
    }

    /// Append a `key=value` pair.
    pub(crate) fn param(mut self, key: &str, value: impl core::fmt::Display) -> Self {
        self.params.append_pair(key, &value.to_string());
        self.empty = false;
        self
    }

    /// Append a `key=value` pair when a value is present.
    pub(crate) fn maybe_param(self, key: &str, value: Option<impl core::fmt::Display>) -> Self {
        match value {
            Some(value) => self.param(key, value),
            None => self,
        }
    }

    /// The path with its encoded query appended — no trailing `?` when
    /// there are no parameters.
    pub(crate) fn finish(mut self) -> String {
        if self.empty {
            self.path
        } else {
            format!("{}?{}", self.path, self.params.finish())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_path_without_params_has_no_trailing_separator() {
        assert_eq!(QueryPath::new("/agents").finish(), "/agents");
        assert_eq!(
            QueryPath::new("/agents")
                .maybe_param("query", None::<&str>)
                .finish(),
            "/agents"
        );
    }

    #[test]
    fn query_path_round_trips_reserved_characters() {
        let values = [
            "#317",
            "texture(observation) & agent(gov.process)",
            "alpha+beta",
            "alpha=beta",
            "100% done",
            "alpha%20beta",
            r#"search("a b")"#,
        ];

        for value in values {
            let path = QueryPath::new("/cognitions")
                .param("query", value)
                .param("limit", 10)
                .finish();
            let (base, query) = path.split_once('?').expect("query is present");
            assert_eq!(base, "/cognitions");
            assert!(!query.contains('#'), "`#` must be escaped in {query}");

            let decoded: Vec<(String, String)> = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect();
            assert_eq!(
                decoded,
                vec![
                    ("query".to_string(), value.to_string()),
                    ("limit".to_string(), "10".to_string()),
                ],
            );
        }
    }
}
