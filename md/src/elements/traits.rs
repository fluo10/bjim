use crate::elements::*;

pub trait ElementLike {
    //fn from_que() -> Self;
    fn to_token_vec(&self) -> Vec<&ParsedToken>;
    fn into_token_vec(self) -> Vec<ParsedToken>;
    fn to_inline_vec(&self) -> Vec<&InlineElement>;
    fn to_inline_vec_mut(&mut self) -> Vec<&mut InlineElement>;
}

