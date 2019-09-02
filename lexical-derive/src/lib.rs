//! Dummy crate to ensure lexical-core works with proc-macros.

#![allow(unused)]

extern crate lexical_core;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Require an associated type and a single value of that type.
#[proc_macro_derive(Lexical)]
pub fn lexical(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl Lexical for #name {
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                ::lexical_core::write(self.value, bytes)
            }

            fn from_lexical(bytes: &[u8])
                -> ::lexical_core::Result<Self>
            {
                Ok(Self { value: ::lexical_core::parse(bytes)? })
            }
        }
    };
    TokenStream::from(expanded)
}
