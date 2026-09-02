/// Marker trait for structs that can be used as annotation payloads.
///
/// A `Notation` is a typed data struct that you attach to a target struct
/// via `#[annotation(MyNotation { ... })]`. Retrieve it later via the
/// `Annotated<T>` trait.
///
/// Derive this on your payload struct:
///
/// ```
/// use noteworthy::Notation;
///
/// #[derive(Notation)]
/// struct Meta {
///     path: &'static str,
///     status: u16,
/// }
/// ```
///
/// The derive macro generates `impl Notation for Meta`, which serves as
/// the gatekeeper — only structs that derive `Notation` can be used as
/// annotation types in `#[annotation(...)]`.
pub trait Notation: Sized + 'static {}

/// Trait for retrieving annotations attached to a type.
///
/// For each `Notation` type `T`, if the target has an
/// `#[annotation(T { ... })]` attribute, the macro generates:
///
/// ```ignore
/// impl Annotated<T> for MyStruct {
///     const DATA: T = T { ... };
/// }
/// ```
///
/// Retrieve via `<MyStruct as Annotated<T>>::DATA` or
/// `MyStruct::annotation::<T>()`.
pub trait Annotated<T: Notation> {
    /// The annotation payload, as a const.
    const DATA: T;
}

/// Extension trait providing ergonomic retrieval.
///
/// Any type that is `Sized` can call `<T>::annotation::<N>()` to retrieve
/// an annotation of type `N`, provided `Annotated<N>` is implemented.
pub trait AnnotatedExt: Sized {
    /// Retrieve the annotation of type `T` attached to this type.
    fn annotation<T: Notation>() -> T
    where
        Self: Annotated<T>,
    {
        <Self as Annotated<T>>::DATA
    }
}

impl<S: Sized> AnnotatedExt for S {}

pub use noteworthy_macros::{Notation, annotation};
