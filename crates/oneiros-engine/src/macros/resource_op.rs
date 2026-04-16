macro_rules! resource_op {
    ($op:ident, $docs:expr) => {{
        let docs = $docs.resource_docs();
        $op.id(docs.nickname.as_str())
            .tag(docs.tag.name.as_str())
            .summary(docs.summary.as_str())
            .description(docs.description.as_str())
    }};
}

pub(crate) use resource_op;
