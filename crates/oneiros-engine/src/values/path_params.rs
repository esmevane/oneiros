use std::marker::PhantomData;

use aide::{
    openapi::{
        Parameter, ParameterData, ParameterSchemaOrContent, PathStyle, ReferenceOr, SchemaObject,
    },
    operation::OperationInput,
};
use schemars::JsonSchema;

/// Declares an `{id}` path parameter for UUID-keyed resource routes.
///
/// Add `.input::<IdPathParam<T>>()` after `resource_op!` in the route closure
/// to document the path parameter in the generated OpenAPI spec. The generic
/// parameter `T` carries type fidelity through to the JSON schema, so the
/// generated TypeScript client sees a typed parameter (e.g. `CognitionId`
/// rather than `string`).
pub(crate) struct IdPathParam<T>(PhantomData<T>);

impl<T: JsonSchema> OperationInput for IdPathParam<T> {
    fn operation_input(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        let schema = ctx.schema.subschema_for::<T>();
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Path {
                parameter_data: ParameterData {
                    name: "id".into(),
                    description: None,
                    required: true,
                    format: ParameterSchemaOrContent::Schema(SchemaObject {
                        json_schema: schema,
                        example: None,
                        external_docs: None,
                    }),
                    deprecated: None,
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                style: PathStyle::Simple,
            }));
    }
}

/// Declares a `{name}` path parameter for name-keyed resource routes.
///
/// Add `.input::<NamePathParam<T>>()` after `resource_op!` in the route closure.
/// The generic parameter `T` carries type fidelity (e.g. `AgentName`).
pub(crate) struct NamePathParam<T>(PhantomData<T>);

impl<T: JsonSchema> OperationInput for NamePathParam<T> {
    fn operation_input(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        let schema = ctx.schema.subschema_for::<T>();
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Path {
                parameter_data: ParameterData {
                    name: "name".into(),
                    description: None,
                    required: true,
                    format: ParameterSchemaOrContent::Schema(SchemaObject {
                        json_schema: schema,
                        example: None,
                        external_docs: None,
                    }),
                    deprecated: None,
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                style: PathStyle::Simple,
            }));
    }
}

/// Declares an `{agent}` path parameter for continuity and pressure routes.
///
/// Add `.input::<AgentPathParam<T>>()` after `resource_op!` in the route closure.
/// Semantically identical to `{name}` but the URL template uses `{agent}`.
pub(crate) struct AgentPathParam<T>(PhantomData<T>);

impl<T: JsonSchema> OperationInput for AgentPathParam<T> {
    fn operation_input(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        let schema = ctx.schema.subschema_for::<T>();
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Path {
                parameter_data: ParameterData {
                    name: "agent".into(),
                    description: None,
                    required: true,
                    format: ParameterSchemaOrContent::Schema(SchemaObject {
                        json_schema: schema,
                        example: None,
                        external_docs: None,
                    }),
                    deprecated: None,
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                style: PathStyle::Simple,
            }));
    }
}

/// Declares a `{ref_key}` path parameter for storage routes.
///
/// Add `.input::<RefKeyPathParam<T>>()` after `resource_op!` in the route closure.
pub(crate) struct RefKeyPathParam<T>(PhantomData<T>);

impl<T: JsonSchema> OperationInput for RefKeyPathParam<T> {
    fn operation_input(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        let schema = ctx.schema.subschema_for::<T>();
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Path {
                parameter_data: ParameterData {
                    name: "ref_key".into(),
                    description: None,
                    required: true,
                    format: ParameterSchemaOrContent::Schema(SchemaObject {
                        json_schema: schema,
                        example: None,
                        external_docs: None,
                    }),
                    deprecated: None,
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                style: PathStyle::Simple,
            }));
    }
}
