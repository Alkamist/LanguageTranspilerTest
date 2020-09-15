use crate::instruction::*;
use crate::variable::VariableType;

pub struct FunctionArgument {
    name: String,
    value_type: VariableType,
    is_self: bool,
    is_reference: bool,
    is_mutable: bool,
}

impl Instruction for FunctionArgument {
    fn to_rust(&self) -> String {
        let mut output = String::with_capacity(25);

        if self.is_self {
            if self.is_reference {
                output.push_str("&");
            }
            if self.is_mutable {
                output.push_str("mut ");
            }

            output.push_str("self");
        }
        else {
            output.push_str(&self.name);
            output.push_str(": ");

            if self.is_reference {
                output.push_str("&");
            }
            if self.is_mutable {
                output.push_str("mut ");
            }

            output.push_str(&self.value_type.to_rust());
        }

        output
    }
}

pub struct FunctionDefinition {
    name: String,
    arguments: String,
    body: String,
    return_type: Option<String>,
    is_public: bool,
}

impl Instruction for FunctionDefinition {
    fn to_rust(&self) -> String {
        let mut output = String::with_capacity(40);

        if self.is_public {
            output.push_str("pub ");
        }

        output.push_str("fn ");
        output.push_str(&self.name);
        output.push_str("(");
        output.push_str(&self.arguments);
        output.push_str(")");

        if let Some(return_type) = &self.return_type {
            output.push_str(" -> ");
            output.push_str(return_type);
        }

        output.push_str(" {\n");
        output.push_str(&self.body);
        output.push_str("}\n");

        output
    }
}
