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
    curr_value: char,       // current literal
    source: String,         // source string
    curr_loc: Location,     // current position 
    next_loc: Location,     // next position 
}

impl Lexer {
    pub fn next(self){
        Self {
        }
    }
}

// 50+50
//  [NUM_LIT,PLUS_SIGN,NUM_LIT]

#[derive(Debug)]
enum TokenType {
    IDENTIFIER,
    ASSIGN_SIGN,
    NUM_LIT,
    PLUS_SIGN,
}


fn main() {
    let script = "10+2";

    let mut expr : Vec<Token>=vec![];

    for c in script.chars() {

            if c.is_digit(10) {
                expr.push(Token::new(TokenType::NUM_LIT,c.to_string())); 
            }

            if c == '+' { 
                expr.push(Token::new(TokenType::PLUS_SIGN,c.to_string())); 
            }

            if c.is_whitespace() { 
                continue; 
            }

    }
    println!("{:?}",expr);
} 
