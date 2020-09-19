use nom::{
    IResult,
    character::complete::{
        space1,
        multispace0,
    },
    sequence::{
        preceded,
    },
    combinator::{
        map,
    },
    multi::{
        separated_list,
    },
    branch::alt,
};

use crate::instruction::*;
use crate::literal::*;
use crate::operator::*;

pub struct Expression {
    pub indentation_level: u32,
    pub indentation_spaces: u32,
    pub body: Vec<Box<dyn Instruction>>,
}

impl Instruction for Expression {
    fn to_rust(&self) -> String {
        let mut output = String::new();
        //output.push_str("{\n");

        //let spaces = (self.indentation_level + 1) * self.indentation_spaces;
        for instruction in &self.body {
            //for _ in 0..spaces {
            //    output.push(' ');
            //}
            output.push_str(&instruction.to_rust());
            //output.push_str(";\n");
        }

        //let spaces = (self.indentation_level) * self.indentation_spaces;
        //for _ in 0..spaces {
        //    output.push(' ');
        //}

        //output.push_str("}\n");

        output
    }
}

impl Expression {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        //let (remain, res) = separated_list(
        //    space1,
        //    alt((
        //        map(Addition::parse, |value| Box::new(value) as Box<dyn Instruction>),
        //        map(Literal::parse, |value| Box::new(value) as Box<dyn Instruction>),
        //    ))
        //)(input)?;

        //let (remain, res) = alt((
        //    map(Addition::parse, |value| Box::new(value) as Box<dyn Instruction>),
        //    map(Literal::parse, |value| Box::new(value) as Box<dyn Instruction>),
        //))(input)?;

        let (remain, res) = parse_addition(input)?;

        let res = vec![res];

        Ok((remain, Self {
            indentation_level: 0,
            indentation_spaces: 4,
            body: res,
        }))
    }
}
