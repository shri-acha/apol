use crate::token::{Token,TokenType};
use std::iter::Peekable;
use std::vec::IntoIter;


#[derive(Debug,PartialEq)]
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
#[derive(Debug,PartialEq)]
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
    pub tok_stream: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new( tok_stream: Vec<Token> )->Self{
        Self{
            tok_stream: tok_stream.into_iter().peekable(),
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
    
    pub fn parse_line(&mut self)-> Option<Node>{
        // take input token and match it with token types
        if let Some(token) = self.tok_stream.peek() {
            match token {
                Token{ tok_type: TokenType::NUM_LIT, ..} =>{
                    return self.handle_number_parsing();
                },
                Token{tok_type: TokenType::PLUS_SIGN, ..} =>{
                    return self.handle_op_parsing();
                },
                _=>{
                    println!("[BEFORE]{:?}",token);
                    self.tok_stream.next();
                    println!("[WAY BEFORE]{:?}",self.tok_stream.peek());
                    return self.parse_line();

                }
            }
        }else {
            return None;
        }
    }

    
    pub fn handle_number_parsing(&mut self)-> Option<Node> {

        // saves the first number as a l_child
        let l_child: Option<Node>; 
        if let Some(token) = self.tok_stream.peek(){
            l_child = Some(Node::new(token.clone(),Box::new(Tree::default())));
        }else {
            return None;
        } 

        // passes to the next token and recursively calls op_parsing 
        self.tok_stream.next();
        if let Some(token) = self.tok_stream.peek(){
            match token{
                Token{tok_type: TokenType::PLUS_SIGN, .. }|
                Token{tok_type: TokenType::SUB_SIGN, .. } |
                Token{tok_type: TokenType::MUL_SIGN, .. } |
                Token{tok_type: TokenType::DIV_SIGN, .. } |
                Token{tok_type: TokenType::ASSIGN_SIGN, .. } 
                =>{
                    return Some(Node::new(token.clone(),Box::new(Tree::new(l_child,self.handle_op_parsing()))));
                },
                _=>{
                    return l_child;
                }
            }
        }else {
                return l_child;
        }
    }

    pub fn handle_op_parsing(&mut self)->Option<Node>{

        // 1 + 2 + 3
        // + { 1, handle_op_parsing() }
        // + { 1, + {2,3} }
        // 
        // 1 + 2

        let l_child: Option<Node>;
        self.tok_stream.next();
        // if the next token exists
        if let Some(token) = self.tok_stream.peek(){
            match token {
                // if the token is a number
                Token{tok_type: TokenType::NUM_LIT,..} => {
                    l_child =  Some(Node::new(token.clone(),Box::new(Tree::default())));
                }
                // if unsure of the token
                _=>{
                    return None;
                }
                }
            }
        // if the next token doesn't exist
        else {
                return None; 
        }

        self.tok_stream.next();

        // if the token(i.e. operator) is present 
        if let Some(token) = self.tok_stream.peek(){ 
            match token {
                Token{tok_type: TokenType::PLUS_SIGN, .. }|
                Token{tok_type: TokenType::SUB_SIGN, .. } |
                Token{tok_type: TokenType::MUL_SIGN, .. } |
                Token{tok_type: TokenType::DIV_SIGN, .. } |
                Token{tok_type: TokenType::ASSIGN_SIGN, .. } 
                    => {
                    return Some(
                        Node::new(token.clone(),
                            Box::new(
                                Tree::new(l_child,self.handle_op_parsing()))
                                )
                            );
                },
                    _=>{ 
                        return  l_child;
                    }
                }
        }else {
            return l_child;
        }
    }
}
