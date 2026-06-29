macro_rules! resource_op {
    ($op:ident, $docs:expr) => {{
        let docs = $docs.resource_docs();
        docs.transform($op)
    }};
}

pub(crate) use resource_op;
