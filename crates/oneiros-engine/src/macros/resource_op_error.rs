macro_rules! resource_op_error {
    ($($t:ty),* $(,)?) => {
        $(
            impl aide::operation::OperationOutput for $t {
                type Inner = ErrorResponse;

                fn operation_response(
                    context: &mut aide::generate::GenContext,
                    _operation: &mut aide::openapi::Operation,
                ) -> Option<aide::openapi::Response> {
                    Some(ErrorResponse::openapi_schema(context))
                }

                fn inferred_responses(
                    context: &mut aide::generate::GenContext,
                    operation: &mut aide::openapi::Operation,
                ) -> Vec<(Option<aide::openapi::StatusCode>, aide::openapi::Response)> {
                    Self::operation_response(context, operation)
                        .into_iter()
                        .map(|r| (None, r))
                        .collect()
                }
            }
        )*
    };
}

pub(crate) use resource_op_error;
