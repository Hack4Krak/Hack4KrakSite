use proc_macro::{Span, TokenStream};
use syn::{ItemStruct, parse_macro_input};

pub fn to_schema_from_file_name(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;
    let fields = &input.fields;
    let generics = &input.generics;

    let span = name.span().unwrap();

    let panic_message = "Could not determine file name for the macro `to_schema_from_file_name`";

    let file_path = Span::file(&span);

    let file_name = file_path
        .split('/')
        .next_back()
        .unwrap_or_else(|| panic!("{}", panic_message))
        .split('.')
        .next()
        .unwrap_or_else(|| panic!("{}", panic_message));

    let file_name_ident = syn::Ident::new(file_name, name.span());

    let output = quote::quote! {
        #[derive(utoipa::ToSchema)]
        #[schema(as = #file_name_ident)]
        #vis struct #name #generics #fields
    };

    output.into()
}
