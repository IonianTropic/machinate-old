use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Object)]
pub fn object(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl Object for #name {
            fn is_atom(&self) -> bool {
                Self != Cons
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
