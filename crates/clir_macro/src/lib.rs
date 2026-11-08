use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn cli(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
