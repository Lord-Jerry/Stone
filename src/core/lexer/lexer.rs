pub mod kinds;
use kinds::{ TokenKind, Token };

// AfriLang lexer
#[allow(dead_code)]
pub struct Lexer {
    // source code
    code: Vec<char>,
    // Lexer position
    position: usize,

    column: usize,
    line: usize,
    // supported space characters
    space_char: String,
    // supported integer literals
    digit_decimal: String,
    // supported characters that can begin an identifier
    identifier_begin_char: String,
    // supported characters that can end or be in the middle of an identifier
    identifier_end_char: String,
    // supported operators
    operator_char: String,
    // supported separators
    separator_char: String,
    // supported keywords
    keywords: Vec<String>,
}

impl Lexer {

    pub fn new(code: String) -> Self {
        Lexer {
            code: code.chars().collect(),
            // lexer position
            position: 0,

            column: 0,
            line: 1,
            space_char: String::from("\t "),
            digit_decimal: String::from("0123456789"),
            identifier_begin_char: String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            identifier_end_char: String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789"),
            operator_char: String::from("+-/=*%"),
            separator_char: String::from("{}[]<>();,"),
            keywords: vec![
                String::from("var"),
                String::from("const"),
                String::from("fn"),
                String::from("if"),
                String::from("else"),
                String::from("elif")
            ],
        }
    }

    // check if lexer position does not exceed code length
    fn is_bound(&self) -> bool {

        if self.position < self.code.len() {
            return true;
        }

        false
    }

    // return character in current lexer position
    fn peek_char(&self) -> Option<char> {

        if self.is_bound() {
            return Some(self.code[self.position]);
        }

        None
    }

    // return current character in lexer position and increment position
    fn eat_char(&mut self) -> char {
        self.position += 1;

        self.code[self.position - 1]
    }

    // check if characters are valid identifier
    fn vaild_identifier(&mut self) -> Token {
        let mut kind = TokenKind::Identifier;
        let mut identifier = String::from("");
        let mut character = self.peek_char();
        let start_postion = self.position;
           
        if self.is_bound() && self.identifier_begin_char.contains(character.unwrap()) {
            identifier.push(self.eat_char());
            // assign new character to character variable 
            character = self.peek_char();

            while self.is_bound() && self.identifier_end_char.contains(character.unwrap()) {
                identifier.push(self.eat_char());

                // assign new character to character variable 
                character = self.peek_char();
            }
        } 

        if identifier.len()  < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;

        Token::new(kind, start_postion, end_position, identifier)

    }

    // check if characters are valid keywords
    fn valid_keyword(&mut self) -> Token {
        let mut keywords = String::from("");
        let mut character = self.peek_char();
        let mut kind = TokenKind::Keywords;
        let start_postion = self.position;

        while self.is_bound() && self.identifier_begin_char.contains(character.unwrap()) {
            keywords.push(self.eat_char());
            character = self.peek_char();
        }

        // keyword does not match the language keyword 
        // revert lexer position and change token kind to unknown
        if !self.keywords.contains(&keywords) {
            self.position = start_postion;
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_postion, end_position, keywords)

        
    }

    // check if character is a valid white space
    fn valid_space(&mut self) -> Token {
        let mut space = String::from("");
        let character = self.peek_char();
        let mut kind = TokenKind::Space;
        let start_postion = self.position;

        if self.is_bound() && self.space_char.contains(character.unwrap()) {
            space = self.eat_char().to_string();
        }

        if space.len() < 1 {
            kind = TokenKind::Unknown;
            self.position = start_postion;
        }

        let end_position = self.position;
        Token::new(kind, start_postion, end_position, space)
    }



    // run all lexer function
    fn lex_next(&mut self) -> Result<Token, TokenKind> {
        let identifier = self.vaild_identifier();
        if identifier.kind != TokenKind::Unknown {
            return Ok(identifier);
        }

        let keyword = self.valid_keyword();
        if keyword.kind != TokenKind::Unknown {
            return Ok(keyword);
        }

        let space = self.valid_space();
        if space.kind != TokenKind::Unknown {
            return Ok(space);
        }

        Err(TokenKind::Unknown)
    }


    pub fn lex(&mut self) {

        while self.is_bound() {

            println!("{:#?}",self.lex_next());
        }
       // println!("{:#?}", self.peek_char());
    }

    

}