use crate::instruction::*;

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
            "float" => Self::Float64,
            "int" => Self::Int32,

            "i8" => Self::Int8,
            "i16" => Self::Int16,
            "i32" => Self::Int32,
            "i64" => Self::Int64,
            "i128" => Self::Int128,

            "u8" => Self::UInt8,
            "u16" => Self::UInt16,
            "u32" => Self::UInt32,
            "u64" => Self::UInt64,
            "u128" => Self::UInt128,

            "size" => Self::Size,
            "usize" => Self::USize,

            "f32" => Self::Float32,
            "f64" => Self::Float64,

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
    pub value: String,
    pub value_type: VariableType,
    pub is_mutable: bool,
    pub infer_type: bool,
}

impl Instruction for VariableDefinition {
    fn to_rust(&self) -> String {
        let mut output = String::with_capacity(30);
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
        output.push_str(&self.value);
        output.push_str(";");
        output
    }
}

pub struct VariableAssignment {
    pub name: String,
    pub value: String,
}

impl Instruction for VariableAssignment {
    fn to_rust(&self) -> String {
        let mut output = String::with_capacity(25);
        output.push_str(&self.name);
        output.push_str(" = ");
        output.push_str(&self.value);
        output.push_str(";");
        output
    }
}
