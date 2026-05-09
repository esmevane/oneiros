use bon::Builder;

use crate::*;

#[derive(Debug, Clone, Builder)]
pub(crate) struct ResourceDocs {
    pub(crate) tag: Tag,
    #[builder(into)]
    pub(crate) nickname: Label,
    #[builder(into)]
    pub(crate) summary: Description,
    #[builder(into)]
    pub(crate) description: Description,
}
