pub enum TokenType {
    Symbol(String),    
    Identifier,
    Keyword,
    Comment,
    Number
}    
pub const VALID_SYMBOLS: &[&str] = &[
    "=", "+", "-", "*", "/", "==", "!=", "<", ">", "<=", ">=", ":", ";", ",",
    "{", "}", "[", "]", "(", ")", "//",
];

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub object: String,
    pub line: u16,
    pub position: u16

}

pub fn create_token(token_type: TokenType, lexeme: String, object: String, line: u16, position: u16) -> Token {
    Token {
        token_type: token_type,
        lexeme: lexeme,
        object: object,
        line: line,
        position: position
    }
}

