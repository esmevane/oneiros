use bon::Builder;

use crate::*;

/// Hints after setting vocabulary — suggest listing or guidebook.
#[derive(Builder)]
pub(crate) struct VocabularyHints {
    pub(crate) kind: String,
}

impl VocabularyHints {
    pub(crate) fn hints(&self) -> Vec<Hint> {
        let kind = &self.kind;
        vec![
            Hint::inspect(format!("{kind} list"), "See all defined {kind}s"),
            Hint::suggest(
                "guidebook <agent>".to_string(),
                "Review how vocabulary shapes cognition",
            ),
        ]
    }
}
