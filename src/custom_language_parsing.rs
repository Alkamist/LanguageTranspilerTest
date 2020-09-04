use std::fs;

use crate::common::char_is_operator;
use crate::variable_definition::*;
use crate::float_literal::*;

//fn process_tokens(tokens: &Vec<String>) -> Vec<ProgramFunction> {
//    let mut function_parser = FunctionParser::new();
//
//    let mut functions: Vec<ProgramFunction> = Vec::new();
//
//    for token in tokens.iter() {
//        if let Some(output_function) = function_parser.parse(token) {
//            functions.push(output_function);
//        }
//    }
//
//    functions
//}

fn split_to_tokens(text: &str) -> Vec<String> {
    let mut text_tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    current_token.clear();

    let chars = text.chars().collect::<Vec<char>>();
    let num_chars = chars.len();
    for (char_id, character) in chars.iter().enumerate() {
        let character = *character;
        let is_empty_line = current_token == "" && character == '\n';

        if character == ' ' || is_empty_line {
            if current_token != "" {
                text_tokens.push(current_token.clone());
                current_token.clear();
            }
        }

        if char_is_operator(character) && !is_empty_line {
            if current_token != "" {
                text_tokens.push(current_token.clone());
                current_token.clear();
            }

            if character == '\n' {
                text_tokens.push(";".to_string());
            }
            else {
                text_tokens.push(character.to_string());
            }
        }
        else if character != '\\' && (character.is_alphanumeric() || character == '_') {
            current_token.push(character);
        }

        if char_id == num_chars - 1 {
            if current_token != "" {
                text_tokens.push(current_token.clone());
                current_token.clear();
                text_tokens.push(";".to_string());
            }
        }
    }

    text_tokens
}

pub fn parse_program(file_path: &str) {
    let file_text = fs::read_to_string(file_path).expect("Could not read the input file!");
    let text_tokens = split_to_tokens(&file_text);

    let mut variable_definition_parser = VariableDefinitionParser::new();
    let mut float_literal_parser = FloatLiteralParser::new();
    //let mut variable_definitions: Vec<VariableDefinition> = Vec::new();

//    for token in text_tokens.iter() {
//        //if let Some(variable_definition) = variable_definition_parser.parse(token) {
//        //    variable_definition.print_debug();
//        //}
//
//        if let Some(float_literal) = float_literal_parser.parse(token) {
//            float_literal.print_debug();
//        }
//    }

    for (token_id, token) in text_tokens.iter().enumerate() {
        if token == "\n" {
            println!("{}: \\n", token_id);
        }
        else {
            println!("{}: {}", token_id, token);
        }
    }
}
