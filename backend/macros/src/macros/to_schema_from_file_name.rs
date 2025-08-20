use proc_macro::{Span, TokenStream};
use syn::{parse_macro_input, DeriveInput};

pub fn derive_to_schema_from_file_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let span = name.span().unwrap();

    let panic_message = "Could not determine file name for the macro `derive_to_schema_from_file_name`";

    let file_path = Span::file(&span);
    let file_name = file_path.split("/").last().unwrap_or_else(|| {;
        panic!("{}", panic_message);
    }).split('.').next().unwrap_or_else(|| {;
        panic!("{}", panic_message);
    });

    let output = quote::quote! {
        impl utoipa::ToSchema for #name {
            fn name() -> Cow<'static, str> {
                let full_type_name = std::any::type_name::<Self>();
                let type_name_without_generic = full_type_name
                    .split_once("<")
                    .map(|(s1, _)| s1)
                    .unwrap_or(full_type_name);
                let type_name = type_name_without_generic
                    .rsplit_once("::")
                    .map(|(_, tn)| tn)
                    .unwrap_or(type_name_without_generic);
                let file_name = #file_name;

                let full_name = format!("{}::{}", file_name, type_name);

                Cow::Borrowed(full_name.as_str())
            }
            fn schema() -> utoipa::openapi::Schema {

            }
        }
    };

    output.into()
}