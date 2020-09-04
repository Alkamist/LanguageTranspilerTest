use crate::common::{
    print_indented,
    ValueType,
    token_is_operator,
    value_type_to_string,
};

#[derive(Clone)]
pub struct VariableDefinition {
    name: String,
    value_type: ValueType,
}

impl VariableDefinition {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value_type: ValueType::Unresolved,
        }
    }

    pub fn name(&self) -> &String { &self.name }
    pub fn set_name(&mut self, value: &str) { self.name = value.to_string() }
    pub fn set_value_type(&mut self, value: ValueType) { self.value_type = value }

    pub fn print_debug(&self) {
        print_indented(0, &format!("Name: {}", self.name));
        print_indented(0, &format!("Value Type: {}", value_type_to_string(self.value_type)));
    }
}

pub struct VariableDefinitionParser {
    parse_id: usize,
    is_parsing: bool,
    output: VariableDefinition,
}

impl VariableDefinitionParser {
    pub fn new() -> Self {
        Self {
            parse_id: 0,
            is_parsing: false,
            output: VariableDefinition::new(),
        }
    }

    pub fn clear(&mut self) {
        self.parse_id = 0;
        self.is_parsing = false;
        self.output = VariableDefinition::new();
    }

    pub fn parse(&mut self, token: &str) -> Option<VariableDefinition> {
        if token == "var" {
            self.is_parsing = true;
            self.parse_id = 0;
        }

        if self.is_parsing {
            if self.parse_id == 1 {
                self.output.set_name(token);
            }

            //if self.parse_id > 1 && !token_is_operator(token) {
            //    self.output.set_name(token);
            //}

            if token == ";" {
                self.is_parsing = false;
                let output = self.output.clone();
                self.clear();
                return Some(output);
            }

            self.parse_id += 1;
        }

        None
    }
}
