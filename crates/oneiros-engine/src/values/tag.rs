use bon::Builder;

use crate::*;

#[derive(Debug, Clone, Builder)]
pub(crate) struct Tag {
    #[builder(into)]
    pub(crate) name: Label,
    #[builder(into)]
    pub(crate) description: Description,
}
