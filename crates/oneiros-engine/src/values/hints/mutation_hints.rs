use bon::Builder;

use crate::*;

/// Hints after a mutation (create/update) — suggest connecting or searching.
#[derive(Builder)]
pub(crate) struct MutationHints {
    pub(crate) ref_token: RefToken,
}

impl MutationHints {
    pub(crate) fn hints(&self) -> Vec<Hint> {
        let ref_token = &self.ref_token;
        vec![
            Hint::suggest(
                format!("connection create <nature> {ref_token} <target>"),
                "Draw a line between related things",
            ),
            Hint::suggest(format!("search {ref_token}"), "Find related entities"),
        ]
    }
}
