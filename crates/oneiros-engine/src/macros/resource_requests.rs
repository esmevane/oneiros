macro_rules! resource_requests {
    ($($request:ident => |$client:ident| { $($inner:tt)* }),* $(,)?) => {
        $(
            impl crate::ClientRequest for $request {
                type Error = crate::ClientError;

                async fn execute_request(
                    &self,
                    client: &crate::Client,
                ) -> Result<Vec<u8>, Self::Error> {
                    let $client = client;
                    $($inner)*
                }
            }
        )*
    };
    ($($request:ident => |$this:ident, $client:ident| { $($inner:tt)* }),* $(,)?) => {
        $(
            impl crate::ClientRequest for $request {
                type Error = crate::ClientError;

                async fn execute_request(
                    &self,
                    client: &crate::Client,
                ) -> Result<Vec<u8>, Self::Error> {
                    let $this = self;
                    let $client = client;
                    $($inner)*
                }
            }
        )*
    };
}

pub(crate) use resource_requests;
