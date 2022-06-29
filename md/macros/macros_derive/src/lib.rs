use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TokenMacro)]
pub fn token_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_token_macro(&ast)
}

fn impl_token_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TokenMacro for #name {
            fn len(&self) -> usize {
                self.literal.len()
            }
        }

        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.literal)
            }
        }
    };
    gen.into()
}