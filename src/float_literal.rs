use crate::common::{
    print_indented,
};

#[derive(Clone)]
pub struct FloatLiteral {
    value: String,
}

impl FloatLiteral {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn value(&self) -> &String { &self.value }
    pub fn set_value(&mut self, value: &str) { self.value = value.to_string() }

    pub fn print_debug(&self) {
        print_indented(0, &format!("Value: {}", self.value));
    }
}

pub struct FloatLiteralParser {
    is_parsing: bool,
    output: FloatLiteral,
    output_string: String,
    one_period_exists: bool,
    is_parsing_invalid_float: bool,
}

impl FloatLiteralParser {
    pub fn new() -> Self {
        Self {
            is_parsing: false,
            output: FloatLiteral::new(),
            output_string: String::new(),
            one_period_exists: false,
            is_parsing_invalid_float: false,
        }
    }

    pub fn clear(&mut self) {
        self.is_parsing = false;
        self.output = FloatLiteral::new();
        self.output_string = String::new();
        self.one_period_exists = false;
        self.is_parsing_invalid_float = false;
    }

    pub fn parse(&mut self, token: &str) -> Option<FloatLiteral> {
        for token_char in token.chars() {
            if let Some(float_literal) = self.parse_char(token_char) {
                return Some(float_literal)
            }
        }

        None
    }

    fn parse_char(&mut self, input_char: char) -> Option<FloatLiteral> {
        if input_char.is_numeric() && !self.is_parsing {
            self.is_parsing = true;
        }

        if self.is_parsing {
            if input_char.is_numeric() {
                self.output_string.push(input_char);
            }
            else if input_char == '.' {
                if self.one_period_exists {
                    self.is_parsing_invalid_float = true;
                }
                else {
                    self.output_string.push(input_char);
                    self.one_period_exists = true;
                }
            }
            else if !self.is_parsing_invalid_float {
                self.output.set_value(&self.output_string);
                let output = self.output.clone();
                self.clear();
                return Some(output);
            }
        }

        None
    }
}
