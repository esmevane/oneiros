/// Generates `impl ResourceRoot for $root` from a simple variant list.
///
/// The dispatch calls each leaf's `Annotated<ResourceMeta>` and
/// `Annotated<ResourceHandler>` impls — the data comes from
/// `#[annotation(...)]` on the leaf structs, not from macro arguments.
///
/// Variants not listed in `operations` get `None` — they're CLI-only or
/// internal operations without HTTP routes or skills.
///
/// # Syntax
///
/// ```ignore
/// resource_root! {
///     ActorRequest => {
///         label: "actors",
///         purpose: "Manage actors within a tenant",
///         operations: [CreateActor, GetActor, ListActors],
///     }
/// }
///
/// // For domains with CLI-only operations that have skills but no routes:
/// resource_root! {
///     ProjectRequest => {
///         label: "projects",
///         purpose: "Manage projects on this host",
///         operations: [CreateProject, GetProject, ListProjects],
///         skills: [ExportProject, ImportProject, ReplayProject],
///     }
/// }
/// ```
macro_rules! resource_root {
    (
        $root:ident => {
            label: $label:literal,
            purpose: $purpose:literal,
            operations: [ $( $leaf:ident ),* $(,)? ]
            $(, skills: [ $( $skill_leaf:ident ),* $(,)? ] )?
            $(, prefix: $prefix:literal)?
            $(,)?
        }
    ) => {
        impl $crate::ResourceRoot for $root {
            const LABEL: &'static str = $label;
            const PURPOSE: &'static str = $purpose;
            $(const PREFIX: &'static str = $prefix;)?

            fn meta_for(kind: <Self as kinded::Kinded>::Kind) -> Option<$crate::ResourceMeta> {
                #[allow(unreachable_patterns)]
                match kind {
                    $(
                        <$root as kinded::Kinded>::Kind::$leaf => Some(<$leaf as $crate::Annotated<$crate::ResourceMeta>>::DATA),
                    )*
                    $(
                        $(
                            <$root as kinded::Kinded>::Kind::$skill_leaf => Some(<$skill_leaf as $crate::Annotated<$crate::ResourceMeta>>::DATA),
                        )*
                    )?
                    _ => None,
                }
            }

            fn handler_for(kind: <Self as kinded::Kinded>::Kind) -> Option<$crate::ResourceHandler> {
                #[allow(unreachable_patterns)]
                match kind {
                    $(
                        <$root as kinded::Kinded>::Kind::$leaf => Some(<$leaf as $crate::Annotated<$crate::ResourceHandler>>::DATA),
                    )*
                    _ => None,
                }
            }
        }
    };
}

pub(crate) use resource_root;
