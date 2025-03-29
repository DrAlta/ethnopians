extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Structs)]
pub fn derive_change_structs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Enum(data_enum) => {
            let struct_definitions = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let struct_name = syn::Ident::new(&format!("Change{}", variant_name), variant_name.span());
                let fields = match &variant.fields {
                    Fields::Unnamed(fields) => {
                        let types = fields.unnamed.iter().map(|field| &field.ty);
                        quote! {
                            #[derive(bevy::prelude::Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                            pub struct #struct_name(#(pub #types),*);
                        }
                    },
                    Fields::Named(fields) => {
                        let named_fields = fields.named.iter().map(|field| {
                            let name = &field.ident;
                            let ty = &field.ty;
                            quote! { pub #name: #ty }
                        });
                        quote! {
                            #[derive(bevy::prelude::Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                            pub struct #struct_name {
                                #(#named_fields),*
                            }
                        }
                    },
                    Fields::Unit => quote! {
                        pub struct #struct_name;
                    },
                };
                fields
            });
            let dispatch_impl = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let struct_name = syn::Ident::new(&format!("Change{}", variant_name), variant_name.span());
                match &variant.fields {
                    Fields::Unnamed(fields) => {
                        let field_names: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("field_{i}"), variant_name.span()))
                            .collect();
                        let args = quote! { #(#field_names.clone()),* };
                        let pattern = quote! { Self::#variant_name(#(#field_names),*) };
                        quote! {
                            #pattern => {
                                let event = #struct_name(#args);
                                commands.send_event(event);
                            }
                        }
                    },
                    Fields::Named(fields) => {
                        let field_names: Vec<_> = fields.named.iter()
                            .map(|field| field.ident.as_ref().unwrap().clone())
                            .collect();
                        let doda = quote! { #(let #field_names = #field_names.clone();)* }; 
                        let pattern = quote! { Self::#variant_name { #(#field_names),* } };
                        let args = quote! { #(#field_names),* };
                        quote! {
                            #pattern => {
                                #doda
                                let event = #struct_name {#args };
                                commands.send_event(event);
                            }
                        }
                    },
                    Fields::Unit => {
                        let pattern = quote! { Self::#variant_name };
                        quote! {
                            #pattern => {
                                let event = #struct_name;
                                commands.send_event(event);
                            }
                        }
                    },
                }
            });
            quote! {
                #(#struct_definitions)*

                impl Dispatch for #name {
                    fn dispatch(&self, commands: &mut Commands) {
                        match self {
                            #(#dispatch_impl),*
                        }
                    }
                }
            }
        }
        _ => panic!("ChangeStructs can only be derived for enums"),
    };

    TokenStream::from(expanded)
}
