use std::fs;

fn print_error(line_number: usize, error: &str) {
    panic!("Line: {}, {}", line_number, error);
}

#[derive(PartialEq, Copy, Clone)]
enum TokenType {
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    BackSlash,
    Star,
    Hash,
    Ampersand,
    VerticalBar,
    Bang,
    BangEquals,
    Equals,
    EqualsEquals,
    Greater,
    GreaterEquals,
    Lesser,
    LesserEquals,
    And,
    Or,
    Identifier,
    String,
    Number,
    Variable,
    Class,
    Function,
    If,
    Else,
    True,
    False,
    Return,
    While,
    For,
    EndOfFile,
}

fn token_type_to_string(token_type: TokenType) -> String {
    match token_type {
        TokenType::LeftParentheses => "LeftParentheses".to_string(),
        TokenType::RightParentheses => "RightParentheses".to_string(),
        TokenType::LeftBrace => "LeftBrace".to_string(),
        TokenType::RightBrace => "RightBrace".to_string(),
        TokenType::Comma => "Comma".to_string(),
        TokenType::Dot => "Dot".to_string(),
        TokenType::Minus => "Minus".to_string(),
        TokenType::Plus => "Plus".to_string(),
        TokenType::Semicolon => "Semicolon".to_string(),
        TokenType::Slash => "Slash".to_string(),
        TokenType::BackSlash => "BackSlash".to_string(),
        TokenType::Star => "Star".to_string(),
        TokenType::Hash => "Hash".to_string(),
        TokenType::Ampersand => "Ampersand".to_string(),
        TokenType::VerticalBar => "VerticalBar".to_string(),
        TokenType::Bang => "Bang".to_string(),
        TokenType::BangEquals => "BangEquals".to_string(),
        TokenType::Equals => "Equals".to_string(),
        TokenType::EqualsEquals => "EqualsEquals".to_string(),
        TokenType::Greater => "Greater".to_string(),
        TokenType::GreaterEquals => "GreaterEquals".to_string(),
        TokenType::Lesser => "Lesser".to_string(),
        TokenType::LesserEquals => "LesserEquals".to_string(),
        TokenType::And => "And".to_string(),
        TokenType::Or => "Or".to_string(),
        TokenType::Identifier => "Identifier".to_string(),
        TokenType::String => "String".to_string(),
        TokenType::Number => "Number".to_string(),
        TokenType::Variable => "Variable".to_string(),
        TokenType::Class => "Class".to_string(),
        TokenType::Function => "Function".to_string(),
        TokenType::If => "If".to_string(),
        TokenType::Else => "Else".to_string(),
        TokenType::True => "True".to_string(),
        TokenType::False => "False".to_string(),
        TokenType::Return => "Return".to_string(),
        TokenType::While => "While".to_string(),
        TokenType::For => "For".to_string(),
        TokenType::EndOfFile => "EndOfFile".to_string(),
    }
}

fn keyword_to_token_type(keyword: &str) -> TokenType {
    match keyword {
        "var" => TokenType::Variable,
        "class" => TokenType::Class,
        "function" => TokenType::Function,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "return" => TokenType::Return,
        "while" => TokenType::While,
        "for" => TokenType::For,
        _ => TokenType::Identifier,
    }
}

struct Token {
    token_type: TokenType,
    text: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, text: String, line: usize) -> Self {
        Self {
            token_type,
            text,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}: {}",
                self.line,
                token_type_to_string(self.token_type),
                self.text)
    }
}

struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
    start_index: usize,
    current_index: usize,
    line_number: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            source_chars: source.chars().collect(),
            tokens: Vec::new(),
            start_index: 0,
            current_index: 0,
            line_number: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start_index = self.current_index;
            self.scan_token();
        }

        self.add_token_with_value(TokenType::EndOfFile, "".to_string());
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParentheses),
            ')' => self.add_token(TokenType::RightParentheses),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '#' => self.add_token(TokenType::Hash),
            '/' => self.add_token(TokenType::Slash),
            '\\' => self.add_token(TokenType::BackSlash),

            '!' => {
                if self.advance_if_next_is('=') { self.add_token(TokenType::BangEquals) }
                else { self.add_token(TokenType::Bang) }
            },
            '=' => {
                if self.advance_if_next_is('=') { self.add_token(TokenType::EqualsEquals) }
                else { self.add_token(TokenType::Equals) }
            },
            '<' => {
                if self.advance_if_next_is('=') { self.add_token(TokenType::LesserEquals) }
                else { self.add_token(TokenType::Lesser) }
            },
            '>' => {
                if self.advance_if_next_is('=') { self.add_token(TokenType::GreaterEquals) }
                else { self.add_token(TokenType::Greater) }
            },
            '&' => {
                if self.advance_if_next_is('&') { self.add_token(TokenType::And) }
                else { self.add_token(TokenType::Ampersand) }
            },
            '|' => {
                if self.advance_if_next_is('|') { self.add_token(TokenType::Or) }
                else { self.add_token(TokenType::VerticalBar) }
            },

            '\n' => self.line_number += 1,

            ' ' => (),
            '\r' => (),
            '\t' => (),

            '"' => self.handle_string(),

            _ => {
                if c.is_numeric() {
                    self.handle_number();
                }
                else if c.is_alphabetic() {
                    self.handle_identifier();
                }
                else {
                    print_error(self.line_number, "Unexpected character.");
                }
            },
        }
    }

    fn advance_if_next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() { false }
        else if self.source_chars[self.current_index] != expected { false }
        else {
            self.current_index += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source_chars[self.current_index - 1]
    }

    fn add_token_with_value(&mut self, token_type: TokenType, value: String) {
        self.tokens.push(Token::new(token_type, value, self.line_number));
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start_index..self.current_index];
        self.tokens.push(Token::new(token_type, text.to_string(), self.line_number));
    }

    fn is_at_end(&self) -> bool {
        return self.current_index >= self.source.len();
    }

    fn peek_char(&self) -> char {
        if self.is_at_end() { '\0' }
        else { self.source_chars[self.current_index] }
    }

    fn peek_next_char(&self) -> char {
        if self.current_index + 1 >= self.source.len() { '\0' }
        else { self.source_chars[self.current_index + 1] }
    }

    fn handle_string(&mut self) {
        while self.peek_char() != '"' && !self.is_at_end() {
            if self.peek_char() == '\n' {
                self.line_number += 1;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            print_error(self.line_number, "Unterminated string.");
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start_index + 1 .. self.current_index - 1].to_string();
        self.add_token_with_value(TokenType::String, value);
    }

    fn handle_number(&mut self) {
        while self.peek_char().is_numeric() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek_char() == '.' && self.peek_next_char().is_numeric() {
            // Consume the "."
            self.advance();

            while self.peek_char().is_numeric() {
                self.advance();
            }
        }

        let value = self.source[self.start_index..self.current_index].to_string();
        self.add_token_with_value(TokenType::Number, value);
    }

    fn handle_identifier(&mut self) {
        while self.peek_char().is_alphanumeric() {
            self.advance();
        }

        let value = self.source[self.start_index..self.current_index].to_string();
        let token_type = keyword_to_token_type(&value);

        if token_type == TokenType::Identifier {
            self.add_token_with_value(token_type, value);
        }
        else {
            self.add_token_with_value(token_type, "".to_string());
        }
    }
}

fn main() {
    let file_text = fs::read_to_string("test_file.txt").expect("Could not read the input file.");
    let mut scanner = Scanner::new(&file_text);

    let tokens = scanner.scan_tokens();

    for token in tokens.iter() {
        println!("{}", token.to_string());
    }
}
