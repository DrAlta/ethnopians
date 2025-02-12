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
                            #[derive(Debug, Clone, bevy::prelude::Event, PartialEq)]
                            pub struct #struct_name(#(#types),*);
                        }
                    },
                    Fields::Named(fields) => {
                        let named_fields = fields.named.iter().map(|field| {
                            let name = &field.ident;
                            let ty = &field.ty;
                            quote! { pub #name: #ty }
                        });
                        quote! {
                            #[derive(Debug, Clone, bevy::prelude::Event, PartialEq)]
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
                let field_names: Vec<_>;
                let struct_name = syn::Ident::new(&format!("Change{}", variant_name), variant_name.span());
                let pattern = match &variant.fields {
                    Fields::Unnamed(fields) => {
                        field_names= (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("field_{i}"), variant_name.span()))
                            .collect();
                        quote! { Self::#variant_name(#(#field_names),*) }
                    },
                    Fields::Named(fields) => {
                        field_names = fields.named.iter()
                            .map(|field| field.ident.as_ref().unwrap().clone())
                            .collect();
                        quote! { Self::#variant_name { #(#field_names),* } }
                    },
                    Fields::Unit => {
                        field_names = Vec::new();
                        quote! { Self::#variant_name }
                    },
                };
                
                let doda = match &variant.fields {
                    Fields::Named(_) => {
                        quote! { #(let #field_names = #field_names.clone();)* }
                    },
                    | Fields::Unnamed(_) => {
                        quote! {}
                    },
                    Fields::Unit => quote! {},
                };
                let args = match &variant.fields {
                    Fields::Named(_) => {
                        quote! { #(#field_names),* }
                    },
                    | Fields::Unnamed(_) => {
                        quote! { #(#field_names.clone()),* }
                    },
                    Fields::Unit => quote! {},
                };
// Todo: Chad wrote this and it only had the code in the unnamed case
// and I didn't boather folding this into the match case above 
// I should do it int furute thou
                match &variant.fields {
                    Fields::Named(..) => {
                        quote! {
                            #pattern => {
                                #doda
                                let event = #struct_name {#args };
                                commands.send_event(event);
                            }
                        }
        
                    },
                    Fields::Unnamed(..) => {
                        quote! {
                            #pattern => {
                                let event = #struct_name(#args);
                                commands.send_event(event);
                            }
                        }
        
                    },
                    Fields::Unit => {
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
