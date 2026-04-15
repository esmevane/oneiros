use askama::Template;

use crate::*;

/// Askama template for rendering hint sections in prompt output.
#[derive(Template)]
#[template(path = "hints.md")]
pub struct HintTemplate<'a> {
    pub hints: &'a [Hint],
}
