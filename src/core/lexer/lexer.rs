//use Token;

#[allow(dead_code)]
// AfriLang lexer
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
    fn vaild_identifier(&mut self) {
        let mut identifier = String::from("");
        let mut character = self.peek_char();
           
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

        println!("{}",identifier);

    }


    pub fn lex(&mut self) {

        while self.is_bound() {
            println!("{:#?}",self.vaild_identifier() );
        }
       // println!("{:#?}", self.peek_char());
    }

    

}