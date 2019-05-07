#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Keywords,
    Unknown,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    start_position: usize,
    end_position: usize,
    token: String,
}

impl Token {
    pub fn new(kind: TokenKind, start_position: usize, end_position: usize, token: String) -> Self {
        Token {
            kind,
            start_position,
            end_position,
            token,
        }
    }
}