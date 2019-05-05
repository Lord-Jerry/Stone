pub enum TokenKind {
    Identifier
}

pub struct Token {
    kind: TokenKind,
    position: usize,
    token: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize, token: Option<String>) {
        self {
            kind,
            position,
            token
        }
    }
}