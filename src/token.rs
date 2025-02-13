#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub raw: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    Plugin,
    Use,
    Prop,
    Enum,
    Type,
    Model,

    // Identifiers
    Identifier,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // Symbols
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenBrace,
    CloseBrace,

    EOF,
}