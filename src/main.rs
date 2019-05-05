//extern crate lexer;
use lexer::{Lexer};
fn main() {
    Lexer::new("hello".to_string()).lex();
    // println!("Hello, world!{}", core);

}

