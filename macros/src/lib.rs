extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_derive(Message)]
pub fn derive_message(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_derive(Resource)]
pub fn derive_resource(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}