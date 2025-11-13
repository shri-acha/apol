use crate::token::{Token,TokenType};
use std::vec::IntoIter;


#[derive(Debug)]
pub struct Tree<'a> {
    pub l_child: Option<Node<'a>>,
    pub r_child: Option<Node<'a>>,
}

impl<'a> Tree<'a> {
    pub fn default()->Self{
        Self{
            l_child:None,
            r_child:None,
        }
    }

    pub fn new(l_child:Option<Node<'a>>,r_child: Option<Node<'a>>)->Self{
        Self{
            l_child,
            r_child
        }
    }
}
#[derive(Debug)]
pub struct Node<'a> {
    pub value: &'a Token,
    pub branch: Box<Tree<'a>>
}

impl<'a> Node<'a> {
    pub fn new(tok_val: &'a Token, branch: Box<Tree<'a>>)->Self{
        Self{
            value: tok_val,
            branch
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub tok_stream: IntoIter<Token>,
    pub curr_token: Option<&'a Token>
}

impl Parser<'_> {
    pub fn new( tok_stream: Vec< Token> )->Self{
        Self{
            tok_stream: tok_stream.into_iter(),
            curr_token:None,
        }
    }
    //   +
    // 2   3
    //  |||
    //  |||
    // 2 + 3
    //
    // cock lol
    //
    // TODO implement a simple AST generation 
    
    pub fn parse_line(&mut self)-> (){
        if let Some(curr_token) = self.curr_token { 
            if curr_token.tok_type == TokenType::BOL {
                let AST = Node::new(
                    curr_token,
                    Box::new(Tree::new(self.consume_exprs(),self.consume_exprs()))
                    );
                println!("{:?}",AST);
            }
        }
    }
    pub fn consume_exprs<'a> (&mut self)-> Option<Node<'a>>{
        println!("{:?}",self.tok_stream);
        None
    }
}
