use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Atomic)]
pub fn atomic(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // Build the output
    let name = &input.ident;
    
    let expanded = quote! {
        use crate::object::Atomic;
        use std::cell::Ref;
        use std::cell::RefMut;
        impl Atomic for #name {
            fn is_atom(&self) -> bool { true }
            fn car(&self) -> Option<Ref<dyn Object>> { None }
            fn cdr(&self) -> Option<Ref<dyn Object>> { None }
            fn car_mut(&self) -> Option<RefMut<dyn Object>> { None }
            fn cdr_mut(&self) -> Option<RefMut<dyn Object>> { None }
        }
    };
    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
