use super::Block;
use super::Header;

pub struct Section{
    header: Option<Header>,
    content: Vec<Block>, 
}