use crate::token::{Token};

pub struct Parser<'a> {
    pub tok_stream: Vec<Token>,
    pub curr_token: Option<&'a Token>
}
impl Parser<'_> {
    pub fn new( tok_stream: Vec< Token> )->Self{
        Self{
            tok_stream,
            curr_token:None,
        }
    }
}
