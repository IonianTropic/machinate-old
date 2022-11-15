use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Object)]
pub fn object(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Build the output
    let name = &input.ident;
    
    let expanded = quote! {
        impl Object for #name {
            fn __atom(&self) -> bool { true }
            fn __car(&self) -> Option<Ref<dyn Object>> { None }
            fn __cdr(&self) -> Option<Ref<dyn Object>> { None }
            fn __car_mut(&self) -> Option<RefMut<dyn Object>> { None }
            fn __cdr_mut(&self) -> Option<RefMut<dyn Object>> { None }
        }
    };
    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
