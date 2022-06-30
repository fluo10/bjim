use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use syn;
use syn::spanned::Spanned;
use convert_case::{Case, Casing};

pub fn impl_enum_is(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Enum(data_enum) => {
            let mut is_variant_functions = TokenStream2::new();
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                let fields_in_variant = match &variant.fields {
                    syn::Fields::Unnamed(_) => quote_spanned! {variant.span() => (..) },
                    syn::Fields::Unit => quote_spanned! {variant.span() => },
                    syn::Fields::Named(_) => quote_spanned! {variant.span() => {..} },
                };
                let mut is_variant_func_name = 
                    format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));
                is_variant_func_name.set_span(variant_name.span());
                is_variant_functions.extend(quote_spanned! {variant.span() => 
                    fn #is_variant_func_name(&self) -> bool {
                        match self {
                            #name::#variant_name #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });
            }
            
            quote! {
                impl #name {
                    #is_variant_functions
                }
            }
        },
        _ => panic!()
    };
    gen.into()
}

pub fn impl_enum_from(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Enum(data_enum) => {
            let mut enum_from_variant_functions = TokenStream2::new();
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                let field_type = if let syn::Fields::Unnamed(x) = &variant.fields {
                    if let Some(y) = x.unnamed.first() {
                        y
                    } else { 
                        panic!()
                    }
                } else {
                    panic!();
                };
                enum_from_variant_functions.extend(quote_spanned! {variant.span() => 
                    impl From<#field_type> for #name {
                        fn from( t: #field_type ) -> Self {
                            #name::#variant_name(t)
                        }
                    }
                });
            }
            
            quote! {
                #enum_from_variant_functions
            }
        },
        _ => panic!()
    };
    gen.into()
}

pub fn impl_enum_get(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Enum(data_enum) => {
            let mut get_variant_functions = TokenStream2::new();
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                let field_type = if let syn::Fields::Unnamed(x) = &variant.fields {
                    if let Some(y) = x.unnamed.first() {
                        y
                    } else { 
                        panic!()
                    }
                } else {
                    panic!();
                };
                //let fields_in_variant = match &variant.fields {
                    //syn::Fields::Unnamed(x) => {
                        //if quote_spanned! {variant.span() => (..) },
                    //syn::Fields::Unit => quote_spanned! {variant.span() => },
                    //syn::Fields::Named(_) => quote_spanned! {variant.span() => {..} },
                //};
                let mut get_variant_func_name = 
                    format_ident!("{}", variant_name.to_string().to_case(Case::Snake));
                get_variant_func_name.set_span(variant_name.span());
                get_variant_functions.extend(quote_spanned! {variant.span() => 
                    fn #get_variant_func_name(&self) -> Option<&#field_type> {
                        match &self {
                            #name::#variant_name(x) => Some(&x),
                            _ => None,
                        }
                    }
                });
            }
            
            quote! {
                impl #name {
                    #get_variant_functions
                }
            }
        },
        _ => panic!()
    };
    gen.into()
}