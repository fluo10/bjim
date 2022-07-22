use crate::token::*;

pub trait TokenLike: AsRef<TokenContent> + AsMut<TokenContent> {
    fn get_literal(&self) -> &str;
    fn get_mut_literal(&mut self) -> &mut str;
    fn get_position(&self) -> Option<&TokenPosition>;
    fn get_mut_position(&mut self) -> Option<&mut TokenPosition>;
    fn len(&self) -> usize;
    fn has_position(&self) -> bool;
    fn take_position(&mut self) -> Option<TokenPosition>;
    fn insert_position(&mut self, p: TokenPosition);
}

impl<T> TokenLike for T where
T: AsRef<TokenContent> + AsMut<TokenContent>,
{
    fn get_literal(&self) -> &str {
        &self.as_ref().literal
    }
    fn get_mut_literal(&mut self) -> &mut str {
        &mut self.as_mut().literal
    }
    fn get_position(&self) -> Option<&TokenPosition> {
        self.as_ref().position.as_ref()
    }
    fn get_mut_position(&mut self) -> Option<&mut TokenPosition> {
        self.as_mut().position.as_mut()
    }
    fn len(&self) -> usize {
        self.as_ref().literal.len()
    }
    fn has_position(&self) -> bool {
        todo!()
    }
    fn take_position(&mut self) -> Option<TokenPosition> {
        todo!()
    }
    fn insert_position(&mut self, p: TokenPosition) {
        self.as_mut().position.insert(p);
    }
}
