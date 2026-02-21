use crate::*;

pub trait AsLink {
    type Linkable: Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError>;
}
