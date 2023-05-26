extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn nih_params_from_faust(_item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {})
}
