use bon::Builder;

use crate::*;

/// Hints after a mutation (create/update) — suggest connecting or searching.
#[derive(Builder)]
pub struct MutationHints {
    pub ref_token: RefToken,
}

impl MutationHints {
    pub fn hints(&self) -> Vec<Hint> {
        let ref_token = &self.ref_token;
        vec![
            Hint::suggest(
                format!("connection create {ref_token} <target>"),
                "Draw a line between related things",
            ),
            Hint::suggest(format!("search {ref_token}"), "Find related entities"),
        ]
    }
}
