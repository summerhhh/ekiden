/// Registers defined RPC methods into the enclave RPC dispatcher.
///
/// # Examples
///
/// This macro should be invoked using a concrete API generated by `rpc_api` as
/// follows:
/// ```
/// with_api! {
///     create_enclave_rpc!(api);
/// }
/// ```
#[macro_export]
macro_rules! create_enclave_rpc {
    (
        metadata {
            name = $metadata_name:ident ;
            version = $metadata_version:expr ;
            client_attestation_required = $client_attestation_required:expr ;
        }

        $(
            rpc $method_name:ident ( $request_type:ty ) -> $response_type:ty ;
        )*
    ) => {
        /// Generate method that will register all RPC methods.
        ///
        /// This method will be automatically invoked when `rpc_init` ECALL is invoked
        /// from the outside.
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __ekiden_rpc_create_enclave() {
            use ekiden_core::error::Result;
            use ekiden_core::rpc::reflection::ApiMethodDescriptor;
            use ekiden_trusted::rpc::dispatcher::{Dispatcher, EnclaveMethod};
            use ekiden_trusted::rpc::request::Request;

            // Register generated methods using the dispatcher.
            let mut dispatcher = Dispatcher::get();
            $(
                dispatcher.add_method(
                    EnclaveMethod::new(
                        ApiMethodDescriptor {
                            name: stringify!($method_name).to_owned(),
                            client_attestation_required: $client_attestation_required,
                        },
                        |request: &Request<$request_type>| -> Result<$response_type> {
                            $method_name(request)
                        },
                    )
                );
            )*
        }
    }
}
