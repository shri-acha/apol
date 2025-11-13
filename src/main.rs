use log::{debug};

#[derive(Debug)]
// assuming single line of code
struct Location {
    col: u64,
    // row: u64,
} 

#[derive(Debug)]
struct Token {
    tok_type: TokenType,
    tok_value: String
}

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
            ch:source.chars().nth(0).unwrap(),
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
                self.read_char();
                Token::new(TokenType::PLUS_SIGN,self.ch.to_string())
            },
            '-'=> {
                self.read_char();
                Token::new(TokenType::PLUS_SIGN,self.ch.to_string())
            },
            '*'=> {
                self.read_char();
                Token::new(TokenType::PLUS_SIGN,self.ch.to_string())
            },
            '/'=> {
                self.read_char();
                Token::new(TokenType::PLUS_SIGN,self.ch.to_string())
            },

            c if c.is_digit(10) => {
                println!("read_number() called!");
                let number_lit: u64 = self.read_number();
                Token::new(TokenType::NUM_LIT,number_lit.to_string())
            },
            '\n'=>{ 
                self.read_char();
                Token::new(TokenType::EOL,self.ch.to_string())
            },                  
            _ => {
                self.read_char();
                Token::new(TokenType::ILLEGAL,self.ch.to_string())
            }// TODO: add operators
        };
        return Some(token);
    }
}

// 50+50
//  [NUM_LIT,PLUS_SIGN,NUM_LIT]

#[derive(Debug)]
enum TokenType {
    IDENTIFIER,
    NUM_LIT,
    ASSIGN_SIGN,
    PLUS_SIGN,
    SUB_SIGN,
    MUL_SIGN,
    DIV_SIGN,
    EOL,
    EOF, // TODO EOF parsing for files 
    ILLEGAL,
}


fn main() {
    let source = String::from("10+10\n");

    let mut lexer = Lexer::new(source);
    println!("{:?}",lexer);
    println!("{:?}",lexer.next());
    println!("{:?}",lexer);
    println!("{:?}",lexer.next());
    println!("{:?}",lexer);
    println!("{:?}",lexer.next());
    println!("{:?}",lexer);
    println!("{:?}",lexer.next());
    println!("{:?}",lexer);
    println!("{:?}",lexer.next());
    println!("{:?}",lexer);
} 
