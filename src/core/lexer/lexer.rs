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

        // check if lexer position hasn't exceeded code length
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
        let start_position = self.position;
           
        // check if lexer position hasn't exceeded code length and if character is a valid aplhabet
        if self.is_bound() && self.identifier_begin_char.contains(character.unwrap()) {
            identifier.push(self.eat_char());
            // assign new character to character variable 
            character = self.peek_char();

            // check if lexer position hasn't exceeded code length and if character is a valid aplhabetnumeric characeter
            while self.is_bound() && self.identifier_end_char.contains(character.unwrap()) {
                identifier.push(self.eat_char());

                // assign new character to character variable 
                character = self.peek_char();
            }
        } 

        // if no avaliable valid character assign token kind to unknown 
        if identifier.len()  < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;

        Token::new(kind, start_position, end_position, identifier)

    }

    // check if characters are valid keywords or boolean literals
    fn valid_keyword(&mut self) -> Token {
        let mut keywords = String::from("");
        let mut character = self.peek_char();
        let mut kind = TokenKind::Keywords;
        let start_position = self.position;

        // check if lexer position hasn't exceeded code length and if character is a valid aplhabet
        while self.is_bound() && self.identifier_begin_char.contains(character.unwrap()) {
            keywords.push(self.eat_char());
            character = self.peek_char();
        }

        // check if characters are valid boolean literal
        if keywords == "true".to_string() || keywords == "false".to_string() {
            kind = TokenKind::Boolean;
            let end_position = self.position;

            return Token::new(kind, start_position, end_position, keywords);
        } 

        // if keyword does not match the language keyword 
        // revert lexer position and change token kind to unknown
        if !self.keywords.contains(&keywords) {
            kind = TokenKind::Unknown;
            // revert lexer position
            self.position = start_position;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, keywords)
        
    }
    // check if characters are valid operator
    fn valid_operator(&mut self) -> Token {
        let mut character = self.peek_char();
        let mut operators = String::from("");
        let mut kind = TokenKind::Operator;
        let start_position = self.position;

        while self.is_bound() && self.operator_char.contains(character.unwrap()) {
            operators.push(self.eat_char());
            character = self.peek_char();
        }

        if operators.len() < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, operators)

    }

    // check if characeter is an identified separator
    fn valid_separator(&mut self) -> Token {
        let mut separator = String::from("");
        let character = self.peek_char();
        let mut kind = TokenKind::Separator;
        let start_position = self.position;

        if self.is_bound() && self.separator_char.contains(character.unwrap()) {
            separator.push(self.eat_char());
        }

        if separator.len() < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, separator)

    }


    // check if character is a valid white space
    fn valid_space(&mut self) -> Token {
        let mut space = String::from("");
        let mut character = self.peek_char();
        let mut kind = TokenKind::Space;
        let start_position = self.position;

        // check if lexer position hasn't exceeded code length and if character is valid space character
        while self.is_bound() && self.space_char.contains(character.unwrap()) {
            space.push(self.eat_char());
            character = self.peek_char();
        }

        // if characeter is not a valid space character assign token kind to unknown
        if space.len() < 1 {
            kind = TokenKind::Unknown;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, space)
    }

    // check single line comment
    fn valid_single_line_comment(&mut self) -> Token {
        let character = self.peek_char();
        let mut comment = String::new();
        let mut kind = TokenKind::Unknown;
        let start_position = self.position;

        if character.unwrap() == '#' {
            comment.push(self.eat_char());

            while self.is_bound() && comment.len() > 0 && self.peek_char().unwrap() != '\n' {
                comment.push(self.eat_char());
            }
            kind = TokenKind::SingleLineComment;
        }

        let end_position = self.position;
        Token::new(kind, start_position, end_position, comment )
    }



    // run all lexer function
    fn lex_next(&mut self) -> Result<Token, Token> {
        let keyword = self.valid_keyword();
        if keyword.kind != TokenKind::Unknown {
            return Ok(keyword);
        }

        let identifier = self.vaild_identifier();
        if identifier.kind != TokenKind::Unknown {
            return Ok(identifier);
        }

        let operator = self.valid_operator();
        if operator.kind != TokenKind::Unknown {
            return Ok(operator);
        }

        let separator = self.valid_separator();
        if separator.kind != TokenKind::Unknown {
            return Ok(separator);
        }

        let space = self.valid_space();
        if space.kind != TokenKind::Unknown {
            return Ok(space);
        }

        let comment = self.valid_single_line_comment();
        if comment.kind != TokenKind::Unknown {
            return Ok(comment);
        }

        Err(Token::new(TokenKind::Unknown, self.position, self.position + 1, self.eat_char().to_string()))
     }


    pub fn lex(&mut self) {

        while self.is_bound() {

            println!("{:#?}",self.lex_next());
        }
       // println!("{:#?}", self.peek_char());
    }

    

}