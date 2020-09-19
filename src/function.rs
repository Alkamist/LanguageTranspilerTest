use crate::instruction::*;
use crate::expression::*;
use crate::variable::VariableType;

pub struct FunctionArgument {
    pub name: String,
    pub value_type: VariableType,
    pub is_self: bool,
    pub is_reference: bool,
    pub is_mutable: bool,
}

impl Instruction for FunctionArgument {
    fn to_rust(&self) -> String {
        let mut output = String::new();

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
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: Expression,
    pub return_type: Option<VariableType>,
    pub is_public: bool,
}

impl Instruction for FunctionDefinition {
    fn to_rust(&self) -> String {
        let mut output = String::new();

        if self.is_public {
            output.push_str("pub ");
        }

        output.push_str("fn ");
        output.push_str(&self.name);
        output.push_str("(");

        let num_arguments = self.arguments.len();
        for (argument_id, argument) in self.arguments.iter().enumerate() {
            output.push_str(&argument.to_rust());
            if argument_id < num_arguments - 1 {
                output.push_str(", ");
            }
        }

        output.push_str(")");

        if let Some(return_type) = &self.return_type {
            output.push_str(" -> ");
            output.push_str(&return_type.to_rust());
        }

        output.push_str(" ");
        output.push_str(&self.body.to_rust());

        output
    }
}
