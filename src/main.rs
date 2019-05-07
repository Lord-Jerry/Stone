//extern crate lexer;
use lexer::{Lexer};
fn main() {
    Lexer::new("var".to_string()).lex();
    // println!("Hello, world!");

}

