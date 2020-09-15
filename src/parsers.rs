use nom::{
    IResult,
    character::complete::{
        char,
        alpha1,
        alphanumeric1,
        one_of,
        multispace0,
        multispace1,
    },
    combinator::{
        recognize,
        opt,
    },
    sequence::{
        pair,
        tuple,
        terminated,
        preceded,
    },
    multi::{
        many0,
        many1,
        separated_list,
    },
    branch::alt,
    bytes::complete::{
        tag,
        is_not,
    },
};

use crate::instruction::*;
use crate::variable::*;
use crate::function::*;

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )
    )(input)
}

pub fn decimal_integer(input: &str) -> IResult<&str, &str> {
    recognize(
        many1(
            terminated(one_of("0123456789"), many0(char('_')))
        )
    )(input)
}

pub fn decimal_float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(
            tuple((
                char('.'),
                decimal_integer,
                opt(tuple((
                    one_of("eE"),
                    opt(one_of("+-")),
                    decimal_integer
                )))
            ))
        ),
        // Case two: 42e42 and 42.42e42
        recognize(
            tuple((
                decimal_integer,
                opt(preceded(
                    char('.'),
                    decimal_integer,
                )),
                one_of("eE"),
                opt(one_of("+-")),
                decimal_integer
            ))
        ),
        // Case three: 42. and 42.42
        recognize(
            tuple((
                decimal_integer,
                char('.'),
                opt(decimal_integer)
            ))
        )
    ))(input)
}

pub fn type_annotation(input: &str) -> IResult<&str, VariableType> {
    let (remain, (_, type_name)) = tuple((
        multispace1,
        identifier,
    ))(input)?;
    Ok((remain, VariableType::from_str(type_name)))
}

pub fn variable_definition(input: &str) -> IResult<&str, Box<dyn Instruction>> {
    let (remain, (declaration, _, name, value_type, _, _, _, value)) = tuple((
        alt((tag("var"), tag("let"))),
        multispace1,
        identifier,
        opt(type_annotation),
        multispace0,
        tag("="),
        multispace0,
        is_not("\r\n"),
    ))(input)?;

    Ok((remain, Box::new(VariableDefinition {
        name: name.to_string(),
        value: value.to_string(),
        value_type:
            if let Some(value_type) = value_type {
                value_type
            }
            else {
                VariableType::Undetermined
            },
        is_mutable:
            if declaration == "var " { true }
            else { false },
        infer_type: true,
    })))
}

pub fn variable_assignment(input: &str) -> IResult<&str, Box<dyn Instruction>> {
    let (remain, (name, _, _, _, value)) = tuple((
        identifier,
        multispace0,
        tag("="),
        multispace0,
        is_not("\r\n"),
    ))(input)?;

    Ok((remain, Box::new(VariableAssignment {
        name: name.to_string(),
        value: value.to_string(),
    })))
}

// pub fn function_definition(input: &str) -> IResult<&str, Box<dyn Instruction>> {
//     let (remain, (declaration, name, _, value_type, _, _, _, value)) = tuple((
//         alt((tag("var "), tag("let "))),
//         identifier,
//         multispace0,
//         opt(type_annotation),
//         multispace0,
//         tag("="),
//         multispace0,
//         is_not("\r\n"),
//     ))(input)?;

//     Ok((remain, Box::new(VariableDefinition {
//         name: name.to_string(),
//         value: value.to_string(),
//         value_type:
//             if let Some(value_type) = value_type {
//                 value_type
//             }
//             else {
//                 VariableType::Undetermined
//             },
//         is_mutable:
//             if declaration == "var " { true }
//             else { false },
//         infer_type: true,
//     })))
// }

pub fn instruction(input: &str) -> IResult<&str, Box<dyn Instruction>> {
    alt((variable_definition, variable_assignment))(input)
}

pub fn test_parse(input: &str) -> IResult<&str, Vec<Box<dyn Instruction>>> {
    preceded(
        multispace0,
        separated_list(
            multispace1,
            instruction,
        ),
    )(input)
}
