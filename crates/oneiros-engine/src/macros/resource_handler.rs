/// Generates `fn route_def() -> ResourceRouteDef` on a leaf inner struct.
/// The transform closure receives an op already set up with docs (via
/// ResourceDocs::transform). The variant-specific type-level calls
/// (response, inputs, security) go in the transform closure.
macro_rules! resource_handler {
    (
        $leaf:ident => {
            handler: $handler:expr,
            method: post,
            transform: |$op:ident| $transform:expr $(,)?
        }
    ) => {
        impl $leaf {
            pub(crate) fn route_def() -> $crate::ResourceRouteDef<$crate::__KindOf<$leaf>> {
                $crate::ResourceRouteDef {
                    build: |kind| {
                        ::aide::axum::routing::post_with($handler, move |$op| {
                            let docs = <<$leaf as $crate::ResourceLeafKind>::Root as $crate::ResourceRoot>::resource_docs(kind.clone());
                            let $op = docs.transform($op);
                            $transform
                        })
                    },
                }
            }
        }

        impl $crate::ResourceLeafRoute for $leaf {
            fn route_def() -> $crate::ResourceRouteDef<$crate::__KindOf<$leaf>> {
                <$leaf>::route_def()
            }
        }
    };

    (
        $leaf:ident => {
            handler: $handler:expr,
            method: get,
            transform: |$op:ident| $transform:expr $(,)?
        }
    ) => {
        impl $leaf {
            pub(crate) fn route_def() -> $crate::ResourceRouteDef<$crate::__KindOf<$leaf>> {
                $crate::ResourceRouteDef {
                    build: |kind| {
                        ::aide::axum::routing::get_with($handler, move |$op| {
                            let docs = <<$leaf as $crate::ResourceLeafKind>::Root as $crate::ResourceRoot>::resource_docs(kind.clone());
                            let $op = docs.transform($op);
                            $transform
                        })
                    },
                }
            }
        }

        impl $crate::ResourceLeafRoute for $leaf {
            fn route_def() -> $crate::ResourceRouteDef<$crate::__KindOf<$leaf>> {
                <$leaf>::route_def()
            }
        }
    };
}

pub(crate) use resource_handler;
