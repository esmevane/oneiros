use bon::Builder;

use crate::*;

/// Hints after setting vocabulary — suggest listing or guidebook.
#[derive(Builder)]
pub struct VocabularyHints {
    pub kind: String,
}

impl VocabularyHints {
    pub fn hints(&self) -> Vec<Hint> {
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
