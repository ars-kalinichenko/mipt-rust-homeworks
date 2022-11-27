#![forbid(unsafe_code)]
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, LitStr};

#[proc_macro_derive(Object, attributes(table_name, column_name))]
pub fn derive_object(input: TokenStream) -> TokenStream {
    // TODO: your code goes here.
    unimplemented!()
}

// TODO: your code goes here.
