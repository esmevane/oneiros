/// User search text translated into a safe FTS5 `MATCH` expression.
///
/// FTS5 parses its `MATCH` operand as a query language: `"` opens phrases,
/// `:` selects columns, `*` and `^` are prefix/initial markers, `-`, `+`
/// and parens are operators, and bare `AND`/`OR`/`NOT`/`NEAR` are keywords.
/// Passing user prose through verbatim turns ordinary text like `#317` or
/// `ref:AAQQ...` into syntax errors.
///
/// We deliberately expose none of that syntax. Each whitespace-separated
/// term becomes an FTS5 string literal (embedded `"` doubled), and terms
/// are joined by spaces — FTS5's implicit AND. Inside a literal the
/// tokenizer still splits on punctuation, so `ref:AAQQ-x` becomes the
/// phrase `ref aaqq x`, matching the same tokens the content was indexed
/// with. bm25 ranking is unaffected.
///
/// Structured querying belongs to lenses, not to FTS5 operators smuggled
/// through a text box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MatchExpression(String);

impl MatchExpression {
    /// Build an expression from raw user text. Returns `None` when the text
    /// has no terms — an empty `MATCH` operand is itself a syntax error, and
    /// "no terms" is better read as "no text constraint".
    pub(crate) fn from_text(text: &str) -> Option<Self> {
        let terms: Vec<String> = text
            .split_whitespace()
            .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
            .collect();

        if terms.is_empty() {
            None
        } else {
            Some(Self(terms.join(" ")))
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(text: &str) -> Option<String> {
        MatchExpression::from_text(text).map(|e| e.as_str().to_string())
    }

    #[test]
    fn quotes_each_term() {
        assert_eq!(
            expr("search relevance").as_deref(),
            Some(r#""search" "relevance""#)
        );
    }

    #[test]
    fn doubles_embedded_quotes() {
        assert_eq!(expr(r#""plumbing"#).as_deref(), Some(r#""""plumbing""#));
        assert_eq!(expr(r#"a"b"#).as_deref(), Some(r#""a""b""#));
    }

    #[test]
    fn operators_become_literals() {
        assert_eq!(expr("#317").as_deref(), Some(r##""#317""##));
        assert_eq!(expr("ref:AAQQ").as_deref(), Some(r#""ref:AAQQ""#));
        assert_eq!(expr("NOT x").as_deref(), Some(r#""NOT" "x""#));
    }

    #[test]
    fn blank_text_has_no_expression() {
        assert_eq!(expr(""), None);
        assert_eq!(expr(" \t\n "), None);
    }
}
