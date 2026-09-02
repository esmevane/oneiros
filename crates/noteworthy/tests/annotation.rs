use noteworthy::Notation;
use noteworthy::annotation;
use noteworthy::{Annotated, AnnotatedExt};

// --- Test notation payloads ---

#[derive(Notation)]
struct Meta {
    path: &'static str,
    summary: &'static str,
    status: u16,
}

#[derive(Notation)]
struct Route {
    method: &'static str,
    handler_name: &'static str,
}

#[derive(Notation)]
struct Client {
    method: &'static str,
    path: &'static str,
}

#[derive(Notation)]
struct Empty {}

// --- Test target structs ---

#[annotation(Meta { path: "/", summary: "Create", status: 201 })]
#[annotation(Route { method: "POST", handler_name: "create" })]
#[annotation(Client { method: "POST", path: "/" })]
struct CreateResource;

#[annotation(Meta { path: "/{id}", summary: "Get", status: 200 })]
#[annotation(Route { method: "GET", handler_name: "get" })]
struct GetResource;

#[annotation(Empty {})]
struct BareResource;

// --- Tests ---

#[test]
fn retrieve_single_annotation() {
    let meta = <CreateResource as Annotated<Meta>>::DATA;
    assert_eq!(meta.path, "/");
    assert_eq!(meta.summary, "Create");
    assert_eq!(meta.status, 201);
}

#[test]
fn retrieve_multiple_annotations_same_struct() {
    let meta = CreateResource::annotation::<Meta>();
    assert_eq!(meta.path, "/");
    assert_eq!(meta.status, 201);

    let route = CreateResource::annotation::<Route>();
    assert_eq!(route.method, "POST");
    assert_eq!(route.handler_name, "create");

    let client = CreateResource::annotation::<Client>();
    assert_eq!(client.method, "POST");
    assert_eq!(client.path, "/");
}

#[test]
fn different_structs_different_annotations() {
    let create_meta = CreateResource::annotation::<Meta>();
    assert_eq!(create_meta.summary, "Create");

    let get_meta = GetResource::annotation::<Meta>();
    assert_eq!(get_meta.summary, "Get");
    assert_eq!(get_meta.path, "/{id}");
    assert_eq!(get_meta.status, 200);

    let get_route = GetResource::annotation::<Route>();
    assert_eq!(get_route.method, "GET");
}

#[test]
fn empty_notation_works() {
    let _empty = BareResource::annotation::<Empty>();
}

#[test]
fn annotation_with_struct_fields() {
    // Annotations use full field syntax (not shorthand)
    let meta = <GetResource as Annotated<Meta>>::DATA;
    assert_eq!(meta.path, "/{id}");
}

// --- Test that notation payloads can themselves be annotated ---

#[derive(Notation)]
struct Wrapper {
    label: &'static str,
}

#[annotation(Wrapper { label: "inner" })]
#[derive(Notation)]
struct NestedNotation {
    value: u16,
}

#[annotation(NestedNotation { value: 42 })]
struct UsesNested;

#[test]
fn nested_notation_types() {
    let wrapper = <NestedNotation as Annotated<Wrapper>>::DATA;
    assert_eq!(wrapper.label, "inner");

    let nested = UsesNested::annotation::<NestedNotation>();
    assert_eq!(nested.value, 42);
}
