/// Generates `impl ResourceRoot for $root` from the source enum and a match
/// mapping kind variants to inner structs.
macro_rules! resource_root {
    (
        $root:ty => {
            meta: { label: $label:literal, summary: $summary:literal $(,)? },
            operations: {
                match $kind_bind:ident => {
                    $( $variant:path => $leaf:ident ),* $(,)?
                }
            }
        }
    ) => {
        impl $crate::ResourceRoot for $root {
            const LABEL: &'static str = $label;
            const PURPOSE: &'static str = $summary;

            fn meta_for(kind: <Self as kinded::Kinded>::Kind) -> $crate::ResourceMeta {
                match kind {
                    $( $variant => <$leaf as $crate::ResourceLeafMeta>::meta(), )*
                }
            }

            fn route_def_for(kind: <Self as kinded::Kinded>::Kind)
                -> $crate::ResourceRouteDef<<Self as kinded::Kinded>::Kind>
            {
                match kind {
                    $( $variant => <$leaf as $crate::ResourceLeafRoute>::route_def(), )*
                }
            }
        }

        $(
            impl $crate::ResourceLeafKind for $leaf
            where
                <$root as kinded::Kinded>::Kind: core::fmt::Display,
            {
                type Kind = <$root as kinded::Kinded>::Kind;
                type Root = $root;
            }
        )*
    };
}

pub(crate) use resource_root;
