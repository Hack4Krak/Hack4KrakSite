use proc_macro::TokenStream;

mod macros;

#[proc_macro_derive(DeriveUpdatableModel)]
pub fn derive_updatable_model(input: TokenStream) -> TokenStream {
    macros::updatable_model::derive_updatable_model_impl(input)
}

#[proc_macro_attribute]
pub fn error_with_messages(_: TokenStream, input: TokenStream) -> TokenStream {
    macros::localized_error::localized_error_impl(input)
}

#[proc_macro_attribute]
pub fn to_schema_from_file_name(_attr: TokenStream, input: TokenStream) -> TokenStream {
    macros::to_schema_from_file_name::to_schema_from_file_name(input)
}
