use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_updatable_model_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let span = name.span().unwrap();

    let panic_message =
        "Could not determine file name for the macro `derive_to_schema_from_file_name`";

    let file_path = Span::file(&span);
    let file_name = file_path
        .split("/")
        .last()
        .unwrap_or_else(|| {
            panic!("{}", panic_message);
        })
        .split('.')
        .next()
        .unwrap_or_else(|| {
            panic!("{}", panic_message);
        });
    let openapi_identifier = syn::Ident::new(file_name, name.span());
    let new_struct_name = syn::Ident::new(&format!("Updatable{name}"), name.span());

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let filtered_fields: Vec<_> = fields
        .iter()
        .filter(|field| !is_primary_key(&field.attrs))
        .collect();

    let new_fields = filtered_fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub #name: Option<#ty>
        }
    });

    let update_stmts = filtered_fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            if let Some(value) = self.#name {
                active_user.#name = sea_orm::ActiveValue::Set(value);
            }
        }
    });

    let output = quote! {
        #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default)]
        #[schema(as = #openapi_identifier)]
        pub struct #new_struct_name {
            #(#new_fields,)*
        }

        impl #new_struct_name {
            pub fn update(self, user: Model) -> ActiveModel {
                let mut active_user: ActiveModel = user.into();
                #(#update_stmts)*

                active_user
            }
        }
    };

    output.into()
}

fn is_primary_key(attributes: &[Attribute]) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident("sea_orm") {
            let mut found = false;
            let _ = attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("primary_key") {
                    found = true;
                }
                Ok(())
            });

            if found {
                return true;
            }
        }
    }
    false
}
