extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_derive(Event)]
pub fn derive_event(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_derive(Resource)]
pub fn derive_resource(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}