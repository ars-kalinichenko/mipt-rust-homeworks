#![forbid(unsafe_code)]
mod generic;
mod labelled;

use generic::impl_generic;
use labelled::impl_labelled;
use proc_macro::TokenStream;

#[proc_macro_derive(Generic)]
pub fn derive_generic(input: TokenStream) -> TokenStream {
    impl_generic(input)
}

#[proc_macro_derive(LabelledGeneric)]
pub fn derive_labelled(input: TokenStream) -> TokenStream {
    impl_labelled(input)
}
