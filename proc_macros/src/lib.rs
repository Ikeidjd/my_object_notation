extern crate proc_macro;

use proc_macro::TokenStream;

mod serializable;

#[proc_macro_derive(Serializable)]
pub fn derive_macro_describe(input: TokenStream) -> TokenStream {
    serializable::derive_proc_macro_impl(input)
}
