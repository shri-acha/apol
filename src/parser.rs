use crate::token::{Token,TokenType};
use std::vec::IntoIter;


#[derive(Debug)]
pub struct Tree{
    pub l_child: Option<Node>,
    pub r_child: Option<Node>,
}

impl Tree {
    pub fn default()->Self{
        Self{
            l_child:None,
            r_child:None,
        }
    }

    pub fn new(l_child:Option<Node>,r_child: Option<Node>)->Self{
        Self{
            l_child,
            r_child
        }
    }
}
#[derive(Debug)]
pub struct Node {
    pub value: Token,
    pub branch: Box<Tree>
}

impl Node {
    pub fn new(tok_val: Token, branch: Box<Tree>)->Self{
        Self{
            value: tok_val,
            branch
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tok_stream: IntoIter<Token>,
}

impl Parser {
    pub fn new( tok_stream: Vec< Token> )->Self{
        Self{
            tok_stream: tok_stream.into_iter(),
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
        let mut curr_token = self.tok_stream.next();

        if let Some(inner_curr_token) = curr_token {

                    if inner_curr_token.tok_type == TokenType::BOL{

                        let AST = Node::new(
                            inner_curr_token,
                            Box::new(Tree::new(self.consume_exprs(),self.consume_exprs()))
                        );
                        println!("{:?}",AST);

                    }
        }
        else {
        }
    }
    pub fn consume_exprs(&mut self)-> Option<Node>{
        let curr_token = self.tok_stream.next();

        if let Some( inner_curr_token)  = curr_token {
            if self.match_literals(& inner_curr_token) {
               return Some(Node::new(inner_curr_token,Box::new(Tree::new(None,None))));
            }
            else if self.match_operators(& inner_curr_token) {
               return Some(Node::new(inner_curr_token,Box::new(Tree::new(self.consume_exprs(),self.consume_exprs()))));
            }
            else{
               return None;
            }
        }else {
            return None;
        }
    }

    pub fn match_literals(&mut self,curr_token: &Token)->bool{
        match &curr_token.tok_type {
            TokenType::NUM_LIT => true,
            _ => false
        }
    }
    pub fn match_operators(&mut self,curr_token:&Token)->bool{
        match &curr_token.tok_type {
            TokenType::ASSIGN_SIGN=>true,
            TokenType::PLUS_SIGN=>true,
            TokenType::SUB_SIGN=>true,
            TokenType::MUL_SIGN=>true,
            TokenType::DIV_SIGN=>true,
            _=>false,
        }
    }
    
}
