use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

const ERRORS_YAML: &str = include_str!("../../../locales/error.yaml");

pub fn localized_error_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let messages: serde_norway::Value = serde_norway::from_str(ERRORS_YAML).expect("Invalid YAML");

    let enum_messages = messages
        .get(enum_name.to_string())
        .cloned()
        .unwrap_or_default();

    let syn::Data::Enum(data_enum) = &input.data else {
        panic!("ErrorWithMessages can only be applied to enums");
    };

    let variants = data_enum.variants.iter().map(|variant| {
        let var_ident = &variant.ident;
        let var_name = var_ident.to_string();

        let is_transparent = variant.attrs.iter().any(|attr| {
            attr.path().is_ident("error")
                && attr.to_token_stream().to_string().contains("transparent")
        });

        let message_attr = if is_transparent {
            quote! {}
        } else {
            let msg = enum_messages
                .get(&var_name)
                .unwrap_or_else(|| panic!("Missing message for variant `{var_name}`"))
                .as_str()
                .expect("Message must be a string");

            quote! {
                #[error(#msg)]
            }
        };

        let fields = &variant.fields;
        let other_attrs = &variant.attrs;
        quote! {
            #(#other_attrs)*
            #message_attr
            #var_ident #fields
        }
    });

    let expanded = quote! {
        #[derive(Debug, thiserror::Error, serde::Serialize)]
        #[serde(tag = "error", content = "details")]
        pub enum #enum_name {
            #(#variants),*
        }
    };

    TokenStream::from(expanded)
}
