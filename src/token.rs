use log::{debug};


#[derive(Debug)]
// assuming single line of code
pub struct Location {
    pub col: u64,
    // row: u64,
} 

#[derive(Debug,Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub tok_value: String
}
    /*
     GRAMMAR 

	 INSTRUCTION : + - * / =  ,
	 BLOCK : (( NUMERIC_LITERAL | IDENTIFIER ) INSTRUCTION BLOCK )
     | ( NUMERIC_LITERAL | IDENTIFIER ) 
    */ 
// TODO have to work on a precedence for operation.

impl Token {
    pub fn new(tok_type: TokenType ,tok_value: String)->Self{
        Self {
            tok_type,
            tok_value
        }
    }
}

// 50+50
//  [NUM_LIT,PLUS_SIGN,NUM_LIT]

#[derive(Debug,PartialEq,Clone)]
pub enum TokenType {
    BOL, // beggining of line
    IDENTIFIER,
    NUM_LIT,
    ASSIGN_SIGN,
    PLUS_SIGN,
    SUB_SIGN,
    MUL_SIGN,
    DIV_SIGN,
    WHITESPACE, // is ignored, but have to create a token for sanity
    EOL,
    EOF, // TODO EOF parsing for files 
    ILLEGAL,
}
