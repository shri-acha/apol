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
            source,
            ch:source.nth(0).unwrap_or(''),
            curr_position: 0,
            read_position: 1,

            location: Location{col:0},
        }
    }
    // consumes a numeric value, possible to be multi characters
    pub fn read_number(self) -> u64{
        while (self.ch.is_digit(10)){
            println!("{:?}",self.ch);
            self.read_char();
        }
    } 

    pub fn read_char(self){
        while(self.ch !='\n'){

        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    pub fn next(&mut self)-> Option<Self::Item>{
        let token = match self.ch {
            '+'=>{
                return Token::new(TokenType::PLUS_SIGN,self.ch)
            }
            c if c.is_digit(10) => {
                let number_lit: u64 = self.read_number();
                Token::new(NUM_LIT,number_lit)
            }
            _=>{None} // TODO: add operators
        }
        Some(Token)
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
    let source = "10+2\n";

    let mut expr : Vec<Token>=vec![];

    for c in source.chars() {

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
