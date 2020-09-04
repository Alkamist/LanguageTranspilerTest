pub fn print_indented(indentation: usize, input: &str) {
    let mut output_string = String::new();
    for _ in 0..indentation {
        output_string.push(' ');
    }
    output_string.push_str(input);
    println!("{}", output_string);
}

#[derive(PartialEq, Copy, Clone)]
pub enum InstructionType {
    FunctionDefinition,
    VariableDefinition,
    Assigment,
    Return,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ValueType {
    Unresolved,
    Float,
    Int,
}

pub fn value_type_to_string(input: ValueType) -> String {
    match input {
        ValueType::Unresolved => "Unresolved".to_string(),
        ValueType::Float => "Float".to_string(),
        ValueType::Int => "Int".to_string(),
    }
}

pub fn token_is_operator(token: &str) -> bool {
    return token == "(" || token == ")" || token == "{"
        || token == "}" || token == "," || token == "="
        || token == ";" || token == "." || token == "\n"
        || token == "+" || token == "-" || token == "*"
        || token == "/"
}

pub fn char_is_operator(token: char) -> bool {
    return token == '(' || token == ')' || token == '{'
        || token == '}' || token == ',' || token == '='
        || token == ';' || token == '.' || token == '\n'
        || token == '+' || token == '-' || token == '*'
        || token == '/'
}