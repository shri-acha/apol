mod token;
mod lexer;
mod parser;

use std::io::{self,Write};
use token::{Token,TokenType};
use lexer::{Lexer};
use parser::{Parser};


fn main() {
    loop {
        print!("\r\n>");
        io::stdout().flush().unwrap();
        for raw_source in io::stdin().lines(){
            let source = String::from(raw_source.unwrap_or(String::from("\n")));

            let mut tokens: Vec::<Token> = vec![];
            tokens.push(Token::new(TokenType::BOL,String::from("BOL"))); // random value for lit
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
