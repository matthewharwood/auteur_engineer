use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PlatSchema)]
pub fn plat_schema_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let name_str = ident.to_string();
    let expanded = quote! {
        impl plat_schema::Schema for #ident {
            fn name() -> &'static str {
                #name_str
            }
        }
    };
    TokenStream::from(expanded)
}
