use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        let current_char = chars.get(0).copied();
        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    pub fn next_token(&mut self) -> Token {
        // if the current character is a whitespace, advance
        while self.current_char.map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }

        let start = self.position;
        let token = match self.current_char {
            Some('{') => Token {token_type: TokenType::OpenBrace, start, end: start + 1, raw: "{".to_string() },
            Some('}') => Token {token_type: TokenType::CloseBrace, start, end: start + 1, raw: "}".to_string() },
            Some('(') => Token {token_type: TokenType::OpenParen, start, end: start + 1, raw: "(".to_string() },
            Some(')') => Token {token_type: TokenType::CloseParen, start, end: start + 1, raw: ")".to_string() },
            Some('[') => Token {token_type: TokenType::OpenSquare, start, end: start + 1, raw: "[".to_string() },
            Some(']') => Token {token_type: TokenType::CloseSquare, start, end: start + 1, raw: "]".to_string() },
            Some(c) if c.is_alphabetic() || c == '_' || c == '@' => self.read_identifier(),
            Some('"') => self.read_string(),
            Some(c) if c.is_digit(10) => self.read_number(),
            Some('-') if self.peek().map_or(false, |c| c.is_digit(10)) => self.read_number(), 
            None => Token { token_type: TokenType::EOF, start: start, end: start, raw: "".to_string() },
            _ => panic!("Unexpected character: {}", self.current_char.unwrap()),
        };
        self.advance();
        token
    }

    fn read_identifier(&mut self) -> Token {
        // Store current position
        let start = self.position;
        let mut raw = String::new();

        if self.current_char == Some('@') {
            self.advance();
        }

        // While the conditions are true advance self
        while self.current_char.map_or(false, |c| c.is_alphanumeric() || c == '.' || c == '_') {
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        Token { token_type: TokenType::Identifier, start, end: self.position, raw}
    }

    fn read_string(&mut self) -> Token {
        let start = self.position;
        // Skip opening ' " '
        self.advance();
        // Create empty string
        let mut value = String::new();
        while let Some(c) = self.current_char {
            if c == '"' { break; }
            value.push(c);
            self.advance();
        }
        Token { token_type: TokenType::StringLiteral, start, end: self.position, raw: value}
    }
    
    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut raw = String::new();

        if self.current_char == Some('-') {
            raw.push('-');
            self.advance();
        }

        while self.current_char.map_or(false, |c| c.is_digit(10)) {
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        let raw = self.input[start..self.position].iter().collect();
        Token { token_type: TokenType::NumberLiteral, start, end: self.position, raw}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_tokens() {
        let mut lexer = Lexer::new("{");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::OpenBrace);
        assert_eq!(token.raw, "{");
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("somethingIdentifier");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "somethingIdentifier");
    }

    #[test]
    fn test_capital_identifier() {
        let mut lexer = Lexer::new("CAPITALIDENTIFIER");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "CAPITALIDENTIFIER");
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"hello\"");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.raw, "hello");
    }

    #[test]
    fn test_number_literal() {
        let mut lexer = Lexer::new("12345");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::NumberLiteral);
        assert_eq!(token.raw, "12345");
    }

    #[test]
    fn test_nested_objects() {
        let mut lexer = Lexer::new("{ options { label \"Test\" value \"X\" } }");
        assert_eq!(lexer.next_token().token_type, TokenType::OpenBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::OpenBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::StringLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::StringLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::CloseBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::CloseBrace);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::EOF);
    }

    #[test]
    fn test_negative_numbers() {
        let mut lexer = Lexer::new("-1234");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::NumberLiteral);
        assert_eq!(token.raw, "-1234");
    }

    #[test]
    fn test_attribute_identifiers() {
        let mut lexer = Lexer::new("@field.input");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "field.input");
    }
}