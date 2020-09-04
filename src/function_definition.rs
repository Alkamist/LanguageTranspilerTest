use crate::common::{
    print_indented,
    ValueType,
    token_is_operator,
    value_type_to_string,
};

#[derive(Clone)]
struct FunctionArgument {
    name: String,
    value_type: ValueType,
    default_value: Option<String>,
}

impl FunctionArgument {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value_type: ValueType::Unresolved,
            default_value: None,
        }
    }

    pub fn set_name(&mut self, value: &str) { self.name = value.to_string() }
    pub fn set_value_type(&mut self, value: ValueType) { self.value_type = value }
    pub fn set_default_value(&mut self, value: &str) { self.default_value = Some(value.to_string()) }

    pub fn print_debug(&self) {
        print_indented(4, &format!("Name: {}", self.name));
        print_indented(4, &format!("Value Type: {}", value_type_to_string(self.value_type)));
        if let Some(value) = &self.default_value {
            print_indented(4, &format!("Default Value: {}", value));
        }
        else {
            print_indented(4, "Default Value: None");
        }
        println!();
    }
}

#[derive(Clone)]
struct FunctionDefinition {
    name: String,
    return_type: ValueType,
    arguments: Vec<FunctionArgument>,
}

impl FunctionDefinition {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            return_type: ValueType::Unresolved,
            arguments: Vec::new(),
        }
    }

    pub fn name(&self) -> &String { &self.name }
    pub fn set_name(&mut self, value: &str) { self.name = value.to_string() }
    pub fn push_argument(&mut self, argument: &FunctionArgument) { self.arguments.push(argument.clone()) }
    pub fn set_return_type(&mut self, value: ValueType) { self.return_type = value }

    pub fn print_debug(&self) {
        print_indented(0, &format!("Name: {}", self.name));
        print_indented(0, &format!("Return Type: {}", value_type_to_string(self.return_type)));
        print_indented(0, &format!("Arguments:"));
        println!();
        for argument in self.arguments.iter() {
            argument.print_debug();
        }
    }
}

struct FunctionDefinitionParser {
    parse_id: usize,
    is_parsing: bool,
    output_function: FunctionDefinition,
}

impl FunctionDefinitionParser {
    pub fn new() -> Self {
        Self {
            parse_id: 0,
            is_parsing: false,
            output_function: FunctionDefinition::new(),
        }
    }

    pub fn clear(&mut self) {
        self.parse_id = 0;
        self.is_parsing = false;
        self.output_function = FunctionDefinition::new();
    }

    pub fn parse(&mut self, token: &str) -> Option<FunctionDefinition> {
        if token == "function" {
            self.is_parsing = true;
            self.parse_id = 0;
        }

        if self.is_parsing {
            if self.parse_id == 1 {
                self.output_function.set_name(token);
            }

            if self.parse_id > 1 && !token_is_operator(token) {
                let mut argument = FunctionArgument::new();
                argument.set_name(token);
                self.output_function.push_argument(&argument);
            }

            if token == ")" {
                self.is_parsing = false;
                let output_function = self.output_function.clone();
                self.clear();
                return Some(output_function);
            }

            self.parse_id += 1;
        }

        None
    }
}
