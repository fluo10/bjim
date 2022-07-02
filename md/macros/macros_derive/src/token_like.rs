use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn;
use syn::spanned::Spanned;

pub fn impl_token_like(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Struct(_) => {
            quote! {
                impl TokenLike for #name {
                    fn len(&self) -> usize {
                        self.literal.len()
                    }
                    fn get_literal(&self) -> &str {
                        &self.literal
                    }
                    fn take_literal(self) -> String {
                        self.literal
                    }
                }
                impl fmt::Display for #name {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "{}", self.literal)
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            let mut len_patterns = TokenStream2::new();
            let mut get_literal_patterns = TokenStream2::new();
            let mut take_literal_patterns = TokenStream2::new();
            for variant in &data_enum.variants {
                let ref variant_name = variant.ident;
                len_patterns.extend(quote_spanned! {variant.span() => 
                    #name::#variant_name(x) => x.len(),
                });
                get_literal_patterns.extend(quote_spanned! {variant.span() => 
                    #name::#variant_name(x) => x.get_literal(),
                });
                take_literal_patterns.extend(quote_spanned! {variant.span() => 
                    #name::#variant_name(x) => x.take_literal(),
                });
            }
            
            quote! {
                impl TokenLike for #name {
                    fn len(&self) -> usize {
                        match self {
                            #len_patterns
                        }
                    }
                    fn get_literal(&self) -> &str {
                        match self {
                            #get_literal_patterns
                        }
                    }
                    fn take_literal(self) -> String {
                        match self {
                            #take_literal_patterns
                        }
                    }
                }
            }
        },
        _ => panic!()
    };
    gen.into()
}
pub fn impl_token_from(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Struct(data_struct) => {
            quote! {
                impl From<impl Into<String>> for #name {
                    fn from(s: impl Into<String>) -> Self {
                        Self{
                            literal: s.into(),
                            ..Default::default()
                        }
                    }
                }
                impl From<(usize, usize, impl Into<String>)> for Self {
                    fn from(s: (usize, usize, impl Into<String>)) for Self {
                        let (line, col, literal) = s;
                        Self{
                            position: (line, col).into(), 
                            literal: s.into(),
                        }
                    }
                }
            }
        },
        _ => panic!()
    };
    gen.into()
}
