pub trait Instruction {
    fn to_rust(&self) -> String;
}
