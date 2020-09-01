enum RawCustomInstruction {
    EmptyLine,
    VariableDefinition(String),
    VariableAssignment(String),
}

pub enum CustomInstruction {
    EmptyLine,
    VariableDefinition {
        is_mutable: bool,
        name: String,
        value: String,
    },
    VariableAssignment {
        name: String,
        value: String,
    },
}

fn create_raw_custom_instruction_from_line(line_id: usize, line: &str) -> RawCustomInstruction {
    let trimmed_line = line.trim_start().trim_end();

    if trimmed_line == "" || trimmed_line == "\n" {
        return RawCustomInstruction::EmptyLine
    }

    for (word_id, word) in trimmed_line.split_whitespace().enumerate() {
        if word_id == 0 {
            match word {

                "let" => return RawCustomInstruction::VariableDefinition(trimmed_line.to_string()),

                _ => ()
            }
        }
        else if word_id == 1 {
            if word == "=" {
                return RawCustomInstruction::VariableAssignment(trimmed_line.to_string());
            }
        }
    }

    panic!(format!("Unknown line type! (Line {})", line_id + 1));
}

fn create_custom_instruction_from_raw_instruction(raw_instructions: &Vec<RawCustomInstruction>, line_id: usize) -> CustomInstruction {
    let raw_instruction = &raw_instructions[line_id];
    match raw_instruction {
        RawCustomInstruction::EmptyLine => CustomInstruction::EmptyLine,

        RawCustomInstruction::VariableDefinition(content) => {
            let mut variable_name = String::new();
            if let Some(name_str) = content.split_whitespace().nth(1) {
                variable_name.push_str(name_str);
            }
            else {
                panic!(format!("Could not determine variable name! (Line {})", line_id + 1));
            }

            let mut variable_value = String::new();
            if let Some(value_str) = content.split("= ").nth(1) {
                variable_value.push_str(value_str);
            }
            else {
                panic!(format!("Could not parse variable value! (Line {})", line_id + 1));
            }

            //let mut is_mutable = true;
            //for search_line_id in line_id..raw_instructions.len() {
            //}

            CustomInstruction::VariableDefinition {
                is_mutable: true,
                name: variable_name,
                value: variable_value,
            }
        },

        RawCustomInstruction::VariableAssignment(content) => {
            let mut variable_name = String::new();
            if let Some(name_str) = content.split_whitespace().nth(0) {
                variable_name.push_str(name_str);
            }
            else {
                panic!(format!("Could not determine variable name! (Line {})", line_id + 1));
            }

            let mut variable_value = String::new();
            if let Some(value_str) = content.split("= ").nth(1) {
                variable_value.push_str(value_str);
            }
            else {
                panic!(format!("Could not parse variable value! (Line {})", line_id + 1));
            }

            CustomInstruction::VariableAssignment {
                name: variable_name,
                value: variable_value,
            }
        },
    }
}

pub fn create_instructions_from_custom_language_string(program_text: &str) -> Vec<CustomInstruction> {
    let mut raw_instructions: Vec<RawCustomInstruction> = Vec::new();
    let mut output_instructions: Vec<CustomInstruction> = Vec::new();

    for (line_id, line) in program_text.lines().enumerate() {
        let raw_instruction = create_raw_custom_instruction_from_line(line_id, line);
        raw_instructions.push(raw_instruction);
    }

    for line_id in 0..raw_instructions.len() {
        let instruction = create_custom_instruction_from_raw_instruction(&raw_instructions, line_id);
        output_instructions.push(instruction);
    }

    output_instructions
}
