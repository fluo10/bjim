mod token_like;
mod enum_is;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(TokenLike)]
pub fn token_like_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    token_like::impl_token_like(&ast)
}

#[proc_macro_derive(EnumIs)]
pub fn enum_is_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    enum_is::impl_enum_is(&ast)
}

#[proc_macro_derive(EnumFrom)]
pub fn enum_from_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    enum_is::impl_enum_from(&ast)
}

#[proc_macro_derive(EnumGet)]
pub fn enum_get_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    enum_is::impl_enum_get(&ast)
}
