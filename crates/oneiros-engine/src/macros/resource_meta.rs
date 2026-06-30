/// Generates `const fn meta() -> ResourceMeta` on a leaf inner struct.
macro_rules! resource_meta {
    (
        $leaf:ident => {
            path: $path:literal,
            summary: $summary:literal,
            description: $description:literal,
            content: $content:expr,
            status: $status:literal $(,)?
        }
    ) => {
        impl $leaf {
            pub(crate) const fn meta() -> $crate::ResourceMeta {
                $crate::ResourceMeta {
                    path: $path,
                    summary: $summary,
                    description: $description,
                    content: $content,
                    status: $status,
                }
            }
        }

        impl $crate::ResourceLeafMeta for $leaf {
            fn meta() -> $crate::ResourceMeta {
                <$leaf>::meta()
            }
        }
    };
}

pub(crate) use resource_meta;
