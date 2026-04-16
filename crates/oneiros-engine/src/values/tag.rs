use bon::Builder;

use crate::*;

#[derive(Debug, Clone, Builder)]
pub struct Tag {
    #[builder(into)]
    pub name: Label,
    #[builder(into)]
    pub description: Description,
}
