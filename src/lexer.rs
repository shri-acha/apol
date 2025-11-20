use crate::token::{Token,TokenType,Location};
use log::debug;
#[derive(Debug)]
pub struct Lexer{
    pub source: String,         // source string
    pub ch : char,              // current literal
    pub curr_position: usize,   // current position
    pub read_position: usize,   // next position
                            // 
                            //
    pub location: Location,     // current location 
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

    // reads char through each line till '\n'
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

