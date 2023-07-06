use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RegisterModel)]
pub fn register_model_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let output = quote! {
        impl TableStruct for #struct_name {
            // Implement trait methods for the struct
        }
    };

    output.into()
}
