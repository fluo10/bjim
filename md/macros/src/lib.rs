pub trait TokenLike {
    fn len(&self) -> usize;
    fn get_literal(&self) -> &str;
    fn take_literal(self) -> String;
}
