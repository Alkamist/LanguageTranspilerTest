use std::fs;

#[derive(PartialEq)]
enum CustomInstructionType {
    EmptyLine,
    ConstantDefinition,
    VariableDefinition,
    VariableAssigment,
}

fn is_empty_line(line_str: &str) -> bool {
    line_str == "" || line_str == "\n"
}

fn is_constant_definition(line_words: &Vec<String>) -> bool {
    line_words[0] == "const"
}

fn is_variable_definition(line_words: &Vec<String>) -> bool {
    line_words[0] == "var"
}

fn is_assignment(line_words: &Vec<String>) -> bool {
    line_words.len() > 1 && line_words[1] == "="
}

struct ProgramLine {
    text: String,
    words: Vec<String>,
    instruction_type: CustomInstructionType,
}

impl ProgramLine {
    pub fn print_type(&self) {
        match self.instruction_type {
            CustomInstructionType::EmptyLine => println!("EmptyLine"),
            CustomInstructionType::ConstantDefinition => println!("ConstantDefinition"),
            CustomInstructionType::VariableDefinition => println!("VariableDefinition"),
            CustomInstructionType::VariableAssigment => println!("VariableAssigment"),
        }
    }

    pub fn instruction_type(&self) -> &CustomInstructionType { &self.instruction_type }
    pub fn words(&self) -> &Vec<String> { &self.words }
}

pub struct Parser {
    program_text: String,
    lines: Vec<ProgramLine>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            program_text: String::new(),
            lines: Vec::new(),
        }
    }

    pub fn read_file(&mut self, file_path: &str) {
        self.program_text = fs::read_to_string(file_path).expect("Could not read the input file!");
    }

    pub fn parse_program_text(&mut self) {
        for (line_id, line) in self.program_text.lines().enumerate() {
            let trimmed_line = line.trim_start().trim_end();
            let line_words: Vec<String> = trimmed_line.split_whitespace()
                                                      .map(|x| x.to_string())
                                                      .collect();

            let instruction_type = {
                if is_empty_line(trimmed_line) { CustomInstructionType::EmptyLine }
                else if is_constant_definition(&line_words) { CustomInstructionType::ConstantDefinition }
                else if is_variable_definition(&line_words) { CustomInstructionType::VariableDefinition }

                // Check and make sure the user is not trying to assign a value to const.
                else if is_assignment(&line_words) {
                    let variable_name = &line_words[0];
                    let mut last_definition_is_const = false;
                    for search_id in 0..line_id {
                        let search_line = &self.lines[search_id];
                        match search_line.instruction_type() {
                            CustomInstructionType::ConstantDefinition => {
                                let search_variable_name = &search_line.words[1];
                                if variable_name == search_variable_name {
                                    last_definition_is_const = true;
                                }
                            },
                            CustomInstructionType::VariableDefinition => {
                                let search_variable_name = &search_line.words[1];
                                if variable_name == search_variable_name {
                                    last_definition_is_const = false;
                                }
                            },
                            _ => ()
                        }
                    }
                    if last_definition_is_const {
                        panic!("Tried to assign a value to const!");
                    }
                    CustomInstructionType::VariableAssigment
                }

                else { panic!(format!("Unknown line type! (Line {})", line_id + 1)); }
            };

            let program_line = ProgramLine {
                text: trimmed_line.to_string(),
                words: line_words,
                instruction_type: instruction_type,
            };

            self.lines.push(program_line);
        }
    }

    //pub fn print_debug_message(&self) {
    //    for line in self.lines.iter() {
    //        line.print_type();
    //    }
    //}
}

//pub fn create_instructions_from_custom_language_string(program_text: &str) -> Vec<CustomInstruction> {
//    let mut raw_instructions: Vec<RawCustomInstruction> = Vec::new();
//    let mut output_instructions: Vec<CustomInstruction> = Vec::new();
//
//    for (line_id, line) in program_text.lines().enumerate() {
//        let raw_instruction = create_raw_custom_instruction_from_line(line_id, line);
//        raw_instructions.push(raw_instruction);
//    }
//
//    for line_id in 0..raw_instructions.len() {
//        let instruction = create_custom_instruction_from_raw_instruction(&raw_instructions, line_id);
//        output_instructions.push(instruction);
//    }
//
//    output_instructions
//}
