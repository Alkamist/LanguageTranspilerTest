use crate::instruction::*;
use crate::expression::*;

#[derive(Debug)]
pub enum VariableType {
    Undetermined,

    Custom(String),

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,

    Size,
    USize,

    Float32,
    Float64,
}

impl VariableType {
    pub fn from_str(type_str: &str) -> Self {
        match type_str {
            "Float" => Self::Float64,
            "Int" => Self::Int32,

            "Int8" => Self::Int8,
            "Int16" => Self::Int16,
            "Int32" => Self::Int32,
            "Int64" => Self::Int64,
            "Int128" => Self::Int128,

            "UInt8" => Self::UInt8,
            "UInt16" => Self::UInt16,
            "UInt32" => Self::UInt32,
            "UInt64" => Self::UInt64,
            "UInt128" => Self::UInt128,

            "Size" => Self::Size,
            "USize" => Self::USize,

            "Float32" => Self::Float32,
            "Float64" => Self::Float64,

            _ => Self::Custom(type_str.to_string())
        }
    }

    pub fn to_rust(&self) -> String {
        match self {
            Self::Int8 => "i8".to_string(),
            Self::Int16 => "i16".to_string(),
            Self::Int32 => "i32".to_string(),
            Self::Int64 => "i64".to_string(),
            Self::Int128 => "i128".to_string(),

            Self::UInt8 => "u8".to_string(),
            Self::UInt16 => "u16".to_string(),
            Self::UInt32 => "u32".to_string(),
            Self::UInt64 => "u64".to_string(),
            Self::UInt128 => "u128".to_string(),

            Self::Size => "size".to_string(),
            Self::USize => "usize".to_string(),

            Self::Float32 => "f32".to_string(),
            Self::Float64 => "f64".to_string(),

            Self::Custom(name) => name.clone(),

            _ => "".to_string()
        }
    }
}

pub struct VariableDefinition {
    pub name: String,
    pub value: Expression,
    pub value_type: VariableType,
    pub is_mutable: bool,
    pub infer_type: bool,
}

impl Instruction for VariableDefinition {
    fn to_rust(&self) -> String {
        let mut output = String::new();
        output.push_str("let ");

        if self.is_mutable {
            output.push_str("mut ");
        }

        output.push_str(&self.name);

        if !self.infer_type {
            output.push_str(": ");
            output.push_str(&self.value_type.to_rust());
        }

        output.push_str(" = ");
        output.push_str(&self.value.to_rust());
        output.push_str(";");

        output
    }
}

pub struct VariableAssignment {
    pub name: String,
    pub value: Expression,
}

impl Instruction for VariableAssignment {
    fn to_rust(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.name);
        output.push_str(" = ");
        output.push_str(&self.value.to_rust());
        output.push_str(";");

        output
    }
}
