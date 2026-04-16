use bon::Builder;

use crate::*;

#[derive(Debug, Clone, Builder)]
pub struct ResourceDocs {
    pub tag: Tag,
    #[builder(into)]
    pub nickname: Label,
    #[builder(into)]
    pub summary: Description,
    #[builder(into)]
    pub description: Description,
}
