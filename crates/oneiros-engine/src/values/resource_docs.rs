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
    #[builder(default, into)]
    pub(crate) path: ResourceRoute,
    #[builder(default, into)]
    pub(crate) content: Content,
}

impl ResourceDocs {
    pub(crate) fn transform<'a>(
        &self,
        op: aide::transform::TransformOperation<'a>,
    ) -> aide::transform::TransformOperation<'a> {
        op.id(self.nickname.as_str())
            .tag(self.tag.name.as_str())
            .summary(self.summary.as_str())
            .description(self.description.as_str())
    }
}
