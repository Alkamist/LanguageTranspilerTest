use nom::{
    IResult,
    character::complete::{
        char,
        alpha1,
        alphanumeric1,
        one_of,
        multispace0,
        multispace1,
        space0,
        space1,
    },
    combinator::{
        recognize,
        opt,
        map_res,
    },
    sequence::{
        pair,
        tuple,
        terminated,
        preceded,
        delimited,
    },
    multi::{
        many0,
        many1,
        separated_list,
    },
    branch::alt,
    bytes::complete::{
        tag,
        take_until,
    },
};

use crate::instruction::*;
use crate::expression::*;
//use crate::operator::*;
//use crate::literal::*;
//use crate::variable::*;
//use crate::function::*;

//pub fn identifier(input: &str) -> IResult<&str, &str> {
//    recognize(
//        pair(
//            alt((alpha1, tag("_"))),
//            many0(alt((alphanumeric1, tag("_")))),
//        )
//    )(input)
//}

//pub fn instruction(input: &str) -> IResult<&str, Box<dyn Instruction>> {
//    alt((
//        literal,
//        variable_definition,
//        variable_assignment,
//        function_definition,
//    ))(input)
//}

//pub fn delimited_expression(input: &str) -> IResult<&str, Vec<Box<dyn Instruction>>> {
//    delimited(
//        tag("{"),
//        separated_list(
//            tag(";"),
//            delimited(
//                multispace0,
//                instruction,
//                multispace0,
//            )
//        ),
//        tag("}")
//    )(input)
//}

//pub fn expression(input: &str) -> IResult<&str, Vec<Box<dyn Instruction>>> {
//    separated_list(
//        tag(";"),
//        delimited(
//            multispace0,
//            instruction,
//            multispace0,
//        )
//    )(input)
//}

//pub fn type_annotation(input: &str) -> IResult<&str, VariableType> {
//    let (remain, (_, _, type_name)) = tuple((
//        tag(":"),
//        space0,
//        identifier,
//    ))(input)?;
//    Ok((remain, VariableType::from_str(type_name)))
//}

// pub fn variable_definition(input: &str) -> IResult<&str, Box<dyn Instruction>> {
//     let (remain, (declaration, _, name, value_type, _, _, _, value, _)) = tuple((
//         alt((tag("var"), tag("let"))),
//         space1,
//         identifier,
//         opt(type_annotation),
//         space0,
//         tag("="),
//         space0,
//         alt((instruction, delimited_expression)),
//         tag(";"),
//     ))(input)?;

//     Ok((remain, Box::new(VariableDefinition {
//         name: name.to_string(),
//         value: value,
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

// pub fn variable_assignment(input: &str) -> IResult<&str, Box<dyn Instruction>> {
//     let (remain, (name, _, _, _, value)) = tuple((
//         identifier,
//         multispace0,
//         tag("="),
//         multispace0,
//         expression,
//     ))(input)?;

//     Ok((remain, Box::new(VariableAssignment {
//         name: name.to_string(),
//         value: value,
//     })))
// }

// pub fn function_argument(input: &str) -> IResult<&str, FunctionArgument> {
//     let (remain, (_, name, var_type)) = tuple((
//         multispace0,
//         identifier,
//         type_annotation,
//     ))(input)?;

//     Ok((remain, FunctionArgument {
//         name: name.to_string(),
//         value_type: var_type,
//         is_self: false,
//         is_reference: false,
//         is_mutable: false,
//     }))
// }

// pub fn function_arguments(input: &str) -> IResult<&str, Vec<FunctionArgument>> {
//     delimited(
//         tag("("),
//         separated_list(
//             tag(","),
//             function_argument,
//         ),
//         tag(")"),
//     )(input)
// }

// pub fn function_definition(input: &str) -> IResult<&str, Box<dyn Instruction>> {
//     let (remain, (_, _, name, _, arguments)) = tuple((
//         tag("function"),
//         multispace1,
//         identifier,
//         multispace0,
//         function_arguments,
//     ))(input)?;

//     Ok((remain, Box::new(FunctionDefinition {
//         name: name.to_string(),
//         arguments: arguments,
//         body: Vec::new(),
//         return_type: None,
//         is_public: false,
//     })))
// }

pub fn test_parse(input: &str) -> IResult<&str, Expression> {
    preceded(
        multispace0,
        Expression::parse,
    )(input)
}
