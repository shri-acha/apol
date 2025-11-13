use log::{debug};
use std::io::{self,Write};

#[derive(Debug)]
// assuming single line of code
struct Location {
    col: u64,
    // row: u64,
} 

#[derive(Debug,Clone)]
struct Token {
    tok_type: TokenType,
    tok_value: String
}
    /*
     GRAMMAR 

	 INSTRUCTION : + - * / =  ,
	 BLOCK : (( NUMERIC_LITERAL | IDENTIFIER ) INSTRUCTION BLOCK )
     | ( NUMERIC_LITERAL | IDENTIFIER ) 
    */
impl Token {
    pub fn new(tok_type: TokenType ,tok_value: String)->Self{
        Self {
            tok_type,
            tok_value
        }
    }
}

#[derive(Debug)]
struct Lexer{
    source: String,         // source string
    ch : char,              // current literal
    curr_position: usize,   // current position
    read_position: usize,   // next position
                            // 
                            //
    location: Location,     // current location 
}


impl Lexer{
    pub fn new(source: String)->Self{
        Self{
            source: source.clone(), // expected setback
            ch:source.chars().nth(0).unwrap_or(' '),
            curr_position: 0,
            read_position: 1,

            location: Location{col:0},
        }
    }
    // consumes a numeric value, possible to be multi characters
    pub fn read_number(&mut self) -> u64{
        let mut number_lit = String::from("");
        while (self.ch.is_digit(10) && self.ch != '\n' ){
            number_lit += &self.ch.to_string();
            self.read_char();
        }
        return number_lit.parse::<u64>().unwrap();
    } 

    // read reads char through each line till '\n'
    pub fn read_char(&mut self){
        
        if self.read_position >= self.source.len(){
            self.ch = '\n';
        }else {
            self.ch = self.source.chars().nth(self.read_position).unwrap();
        }
        self.curr_position = self.read_position;
        self.read_position = self.curr_position + 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self)-> Option<Self::Item>{
        let token = match self.ch {
            '+'=> {
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::PLUS_SIGN,ch_buf.to_string())
            },
            '-'=> {
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::SUB_SIGN,ch_buf.to_string())
            },
            '*'=> {
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::MUL_SIGN,ch_buf.to_string())
            },
            '/'=> {
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::DIV_SIGN,ch_buf.to_string())
            },

            c if c.is_digit(10) => {
                debug!("read_number() called!");
                let number_lit: u64 = self.read_number();
                Token::new(TokenType::NUM_LIT,number_lit.to_string())
            },
            '\n'=>{ 
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::EOL,ch_buf.to_string())
            },
            ' '=>{
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::WHITESPACE,ch_buf.to_string()) // value gets dunked
            } // ignore whitespaces
            _ => {
                let ch_buf = self.ch;
                self.read_char();
                Token::new(TokenType::ILLEGAL,ch_buf.to_string())
            }// TODO: add operators
        };
        if token.tok_type != TokenType::WHITESPACE {
            return Some(token);
        }else{
            return None;
        }
    }
}

// 50+50
//  [NUM_LIT,PLUS_SIGN,NUM_LIT]

#[derive(Debug,PartialEq,Clone)]
enum TokenType {
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

struct Parser<'a> {
    tok_stream: Vec<Token>,
    curr_token: &'a Token,
}


// takes in a token stream and create an ast
impl Parser{
    pub fn new(tok_stream: Vec<Token>,)->Self{

    }
}


fn main() {
    loop {
        print!("\r\n>");
        io::stdout().flush().unwrap();
        for raw_source in io::stdin().lines(){
            let source = String::from(raw_source.unwrap_or(String::from("\n")));

            let mut tokens: Vec::<Token> = vec![];
            let mut lexer = Lexer::new(source);

            loop {
                if let Some(token) = lexer.next(){
                    println!("{:?}",token);
                    tokens.push(token.clone());
                    if token.tok_type == TokenType::EOL {
                        break;
                    }
                } 
            }
        }
    }
} 
