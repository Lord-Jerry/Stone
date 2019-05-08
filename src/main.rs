//extern crate lexer;
use lexer::{Lexer};
fn main() {
    Lexer::new("let character1 = true; ".to_string()).lex();
    // println!("Hello, world!");

}
